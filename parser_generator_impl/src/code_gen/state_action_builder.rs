use std::collections::HashMap;

use cknittel_util::{iter::JoinWith, proc_macro_util::collect_tokens::TryCollectTokens};
use itertools::Itertools;
use lr_table::{
  grammar::ProductionRuleIndex,
  indexed_grammar::ProductionRuleId,
  lr_state_map::LRStateMap,
  lr_table::{Action, StateId},
  vocabulary::AugmentedVocabToken,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::{GrammarInfo, TerminalType},
    terminal::{PatternMode, UserDefinedSymbol},
  },
  code_gen::{
    constructor::build_constructor,
    reduce_rule::{apply_goto, bind_production_nodes_to_locals},
    states_enum::{enum_name, qualified_enum_variant_name},
    util::{TokenStreamResult, unique_prefixed_ident},
  },
};

pub fn root_production_type(grammar_info: &GrammarInfo) -> proc_macro2::TokenStream {
  let grammar = grammar_info.grammar();

  let root_label = grammar.orig_production_label(grammar.root_production_label());
  match grammar_info.label_type(root_label) {
    Some(root_type) => quote! { #root_type },
    None => quote! { () },
  }
}

pub fn state_action_function_name(state_id: StateId) -> syn::Ident {
  syn::Ident::new(&format!("parse_s{}", state_id.id()), Span::call_site())
}

fn token_matcher(
  token: &AugmentedVocabToken<UserDefinedSymbol>,
  grammar_info: &GrammarInfo,
  mode: PatternMode,
) -> TokenStreamResult {
  match token {
    AugmentedVocabToken::Token(token) => {
      grammar_info.terminal_type().try_build_matcher(token, mode)
    }
    AugmentedVocabToken::EndOfStream => Ok(quote! { None }),
    AugmentedVocabToken::Epsilon => unreachable!(),
  }
}

#[derive(Default)]
struct CollectLikeActions {
  reduce_map: HashMap<ProductionRuleId, Vec<AugmentedVocabToken<UserDefinedSymbol>>>,
  shift_map: Vec<(AugmentedVocabToken<UserDefinedSymbol>, StateId)>,
  accept: Option<AugmentedVocabToken<UserDefinedSymbol>>,
}

impl CollectLikeActions {
  fn peeked_val_ident() -> proc_macro2::Ident {
    proc_macro2::Ident::new("__peeked_val", Span::call_site())
  }

  /// Collects all actions from this state into groups which can be processed
  /// together, which leads to much smaller assembly code compared to simply
  /// matching on each token and applying some action.
  fn build_for_state(state_id: StateId, grammar_info: &GrammarInfo) -> Self {
    let mut action_map = CollectLikeActions::default();

    for (token, action) in grammar_info
      .lr_table()
      .state_actions(state_id, grammar_info.grammar())
    {
      match action {
        Action::Shift { next_state } => action_map.shift_map.push((token, *next_state)),
        Action::Reduce { rule } => action_map.reduce_map.entry(*rule).or_default().push(token),
        Action::Accept => {
          debug_assert!(action_map.accept.is_none());
          action_map.accept = Some(token);
        }
      }
    }

    action_map
  }

  /// We forbid all ambiguous overlapping patterns to make grammars easier to
  /// reason about. This is a runtime check done on all tokens to verify that
  /// they match at most one pattern in the pattern list.
  ///
  /// The check is only enabled if the `debug_assertions` config is enabled.
  fn debug_check_token_conflict(&self, grammar_info: &GrammarInfo) -> TokenStreamResult {
    // We only need to check for pattern conflicts for enum terminals. We don't
    // support patterns for raw terminals.
    if matches!(grammar_info.terminal_type(), TerminalType::Raw(_)) {
      return Ok(quote! {});
    }

    let mut all_tokens = self
      .reduce_map
      .values()
      .flatten()
      .chain(self.shift_map.iter().map(|(tokens, _)| tokens))
      .chain(self.accept.as_ref())
      .filter(|token| token.token().is_some())
      .peekable();
    if all_tokens.peek().is_none() {
      return Ok(quote! {});
    }

    let peeked_val = Self::peeked_val_ident();

    let count_matches = all_tokens
      .map(|token| {
        let matcher = token_matcher(token, grammar_info, PatternMode::Unnamed)?;

        Ok(quote! { (matches!(#peeked_val, #matcher) as usize) })
      })
      .join_with(|| Ok(quote! { + }))
      .try_collect_tokens()?;

    Ok(quote! {
      #[cfg(debug_assertions)]
      {
        let count_matches = #count_matches;
        if count_matches > 1 {
          return Err(
            ::parser_generator::error::ParserError::overlapping_token_matchers(format!(
              "{:?}",
              #peeked_val
            )),
          );
        }
      }
    })
  }

  /// Produces the match arms for tokens which should be shift actions. These
  /// matches don't return, instead yielding the next state that should be
  /// pushed to the stack.
  fn shift_match_and_yield(&self, grammar_info: &GrammarInfo) -> TokenStreamResult {
    self
      .shift_map
      .iter()
      .map(|(token, next_state)| {
        let matcher = token_matcher(token, grammar_info, PatternMode::Named)?;
        let next_state_name = qualified_enum_variant_name(*next_state, grammar_info);
        let token = grammar_info.terminal_type().try_build_pattern(
          token.token().expect(
            "CollectLikeActions::shift_match_and_yield: shift_map should not contain epsilon/eof",
          ),
          PatternMode::Rhs,
        )?;
        Ok(quote! {
          #matcher => #next_state_name(#token),
        })
      })
      .try_collect_tokens()
  }

  /// Produces the match arms for tokens which should be reduce actions. These
  /// matches will always return.
  ///
  /// All tokens which reduce using the same rule will be combined into a
  /// single match conjoined with `|`.
  fn reduce_match_and_return(
    &self,
    state_id: StateId,
    grammar_info: &GrammarInfo,
    state_map: &LRStateMap<UserDefinedSymbol>,
  ) -> TokenStreamResult {
    self
      .reduce_map
      .iter()
      .map(|(rule, tokens)| {
        // Generates a matcher for all tokens in this set, e.g.
        // `token1 | token2 | ...`
        let match_any_token = tokens
          .iter()
          .map(|token| {
            let matcher = token_matcher(token, grammar_info, PatternMode::Unnamed)?;
            Ok(quote! {
              #matcher
            })
          })
          .join_with(|| Ok(quote! { | }))
          .try_collect_tokens()?;

        let rule = grammar_info.grammar().production_rule(*rule);
        let (extract_vars, next_states) =
          bind_production_nodes_to_locals(state_id, rule, grammar_info, state_map);
        let goto = apply_goto(
          rule.original_index(),
          *rule.symbol(),
          next_states,
          grammar_info,
        )?;
        Ok(quote! {
          #match_any_token => {
            #extract_vars
            #goto
            return Ok(::parser_generator::parser_state::ParserControl::Continue);
          }
        })
      })
      .try_collect_tokens()
  }

  /// Generates an accept matcher, if one is needed, which constructs the final
  /// result and returns it immediately.
  fn accept_match_and_return(
    &self,
    state_id: StateId,
    grammar_info: &GrammarInfo,
    state_map: &LRStateMap<UserDefinedSymbol>,
  ) -> TokenStreamResult {
    let Some(accept_token) = &self.accept else {
      return Ok(TokenStream::new());
    };

    let matcher = token_matcher(accept_token, grammar_info, PatternMode::Unnamed)?;

    let root_rule_id = grammar_info.grammar().root_production_rule();
    let root_rule = grammar_info.grammar().production_rule(root_rule_id);
    let (extract_vars, next_states) =
      bind_production_nodes_to_locals(state_id, root_rule, grammar_info, state_map);
    debug_assert_eq!(
      next_states.into_iter().collect_vec(),
      vec![grammar_info.lr_table().root_state()]
    );
    let constructor = build_constructor(ProductionRuleIndex(0), grammar_info)?;
    let state = unique_prefixed_ident("state");
    Ok(quote! {
      #matcher => {
        #extract_vars
        #state.verify_empty_stack();
        return Ok(::parser_generator::parser_state::ParserControl::Accept(#constructor));
      }
    })
  }

  /// Generates code to match the next token from the stream, apply the right
  /// action, and return the `ParserResult` for this action.
  fn generate_actions(
    &self,
    state_id: StateId,
    grammar_info: &GrammarInfo,
    state_map: &LRStateMap<UserDefinedSymbol>,
  ) -> TokenStreamResult {
    let peeked_val = Self::peeked_val_ident();
    let state = unique_prefixed_ident("state");
    let peek_next = quote! {
      #state.stream().peek_next().map(|token| match token {
        Ok(token) => Ok(token.borrow()),
        Err(err) => Err(err.clone().into_user_error()),
      }).transpose()?
    };

    let debug_check_token_conflict = self.debug_check_token_conflict(grammar_info)?;

    let reduce_matches = self.reduce_match_and_return(state_id, grammar_info, state_map)?;
    let accept_matches = self.accept_match_and_return(state_id, grammar_info, state_map)?;

    let return_err = quote! {
      Some(peeked_token) => return Err(::parser_generator::error::ParserError::new(
        Some(peeked_token.clone()),
        ::std::vec::Vec::new()
      )),
      None => return Err(::parser_generator::error::ParserError::new(
        None,
        ::std::vec::Vec::new()
      )),
    };

    if self.shift_map.is_empty() {
      Ok(quote! {
        let #peeked_val = #peek_next;
        #debug_check_token_conflict

        match #peeked_val {
          #reduce_matches
          #accept_matches
          #return_err
        }
      })
    } else {
      let shift_matches = self.shift_match_and_yield(grammar_info)?;
      let state = unique_prefixed_ident("state");

      Ok(quote! {
        let #peeked_val = #peek_next;
        #debug_check_token_conflict

        let next_state = match #peeked_val {
          #shift_matches
          #reduce_matches
          #accept_matches
          #return_err
        };

        #state.stream_mut().advance();
        #state.push(next_state);
        Ok(::parser_generator::parser_state::ParserControl::Continue)
      })
    }
  }
}

pub fn generate_state_action_function(
  state_id: StateId,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap<UserDefinedSymbol>,
) -> TokenStreamResult {
  let token_type = grammar_info.terminal_type().inner_type();
  let enum_name = enum_name(grammar_info);
  let fn_name = state_action_function_name(state_id);
  let result_type = root_production_type(grammar_info);
  let error_type = grammar_info.error_type();

  let action_map = CollectLikeActions::build_for_state(state_id, grammar_info);
  let actions = action_map.generate_actions(state_id, grammar_info, state_map)?;

  let state = unique_prefixed_ident("state");

  Ok(quote! {
    fn #fn_name<
      I,
      B: ::std::borrow::Borrow<#token_type>,
      E: ::parser_generator::error::ParserUserErrorOrInfallible<#token_type, #error_type> + Clone
    >(
      #state: &mut ::parser_generator::parser_state::ParserState<::core::result::Result<B, E>, #enum_name, I>
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<#result_type>,
      #token_type,
      #error_type,
    >
    where
      I: Iterator<Item = ::core::result::Result<B, E>>,
    {
      #actions
    }
  })
}
