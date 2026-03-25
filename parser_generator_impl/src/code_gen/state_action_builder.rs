use std::{collections::HashSet, str::FromStr};

use cknittel_util::proc_macro_util::collect_tokens::{CollectTokens, TryCollectTokens};
use lr_table::{
  indexed_grammar::{IndexedGrammar, IndexedProductionRule},
  lr_state_map::LRStateMap,
  lr_table::{Action, LRTable, StateId},
  vocabulary::AugmentedVocabToken,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{
  annotated_grammar::{parse_grammar::GrammarInfo, production_ref::ProductionRefName},
  code_gen::{
    states_enum::{enum_name, qualified_enum_variant_name},
    util::TokenStreamResult,
  },
  ParserGeneratorError, ParserGeneratorResult,
};

fn try_build_token(text: &str) -> ParserGeneratorResult<proc_macro2::Literal> {
  proc_macro2::Literal::from_str(text)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, proc_macro::Span::call_site()))
}

pub fn root_production_type(
  grammar: &IndexedGrammar<String, ProductionRefName>,
  grammar_info: &GrammarInfo,
) -> proc_macro2::TokenStream {
  let root_label = grammar.orig_production_label(grammar.root_production_label());
  match grammar_info.label_type(root_label) {
    Some(root_type) => quote! { #root_type },
    None => quote! { () },
  }
}

pub fn state_action_function_name(state_id: StateId) -> syn::Ident {
  syn::Ident::new(
    &format!("parse_s{}", state_id.id()),
    proc_macro2::Span::call_site(),
  )
}

fn token_matcher(token: &AugmentedVocabToken<String>) -> TokenStreamResult {
  Ok(match token {
    AugmentedVocabToken::Token(token) => {
      let token = try_build_token(&token)?;
      quote! { Some(&#token) }
    }
    AugmentedVocabToken::EndOfStream => quote! { None },
    AugmentedVocabToken::Epsilon => unreachable!(),
  })
}

fn extract_state(
  state_ids: impl IntoIterator<Item = StateId>,
  grammar_info: &GrammarInfo,
) -> TokenStream {
  let match_arms = state_ids
    .into_iter()
    .map(|state_id| {
      let matcher = qualified_enum_variant_name(state_id, grammar_info);
      quote! {
        #matcher(v) => v,
      }
    })
    .collect_tokens();

  quote! {
    match state.pop_state() {
      #match_arms
      _ => unsafe { ::std::hint::unreachable_unchecked() }
    }
  }
}

fn bind_production_node(
  node_index: usize,
  state_ids: impl IntoIterator<Item = StateId>,
  grammar_info: &GrammarInfo,
) -> TokenStream {
  let var_ident = syn::Ident::new(&format!("__v{node_index}"), Span::call_site());
  let extract_state = extract_state(state_ids, grammar_info);
  quote! {
    let #var_ident = #extract_state;
  }
}

fn stack_states_for_rule(
  state_id: StateId,
  rule: &IndexedProductionRule,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap,
) -> TokenStream {
  let mut states = HashSet::new();
  states.insert(state_id);
  let rule_len = rule.rule().len();

  std::iter::once(bind_production_node(rule_len - 1, [state_id], grammar_info))
    .chain((0..rule_len - 1).rev().map(|node_index| {
      states = states
        .iter()
        .fold(HashSet::new(), |mut next_states, state_id| {
          next_states.extend(state_map.back_edges(*state_id));
          next_states
        });
      bind_production_node(node_index, states.iter().cloned(), grammar_info)
    }))
    .collect_tokens()
}

fn apply_action(
  token: &AugmentedVocabToken<String>,
  action: &Action,
  state_id: StateId,
  grammar: &IndexedGrammar<String, ProductionRefName>,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap,
) -> TokenStreamResult {
  let enum_name = qualified_enum_variant_name(state_id, grammar_info);
  Ok(match action {
    Action::Shift { next_state } => {
      let next_state_name = qualified_enum_variant_name(*next_state, grammar_info);
      let token = try_build_token(token.token().unwrap())?;
      quote! {
        state.push(#next_state_name(#token));
        Ok(::parser_generator::parser_state::ParserControl::Continue)
      }
    }
    Action::Reduce { rule } => {
      let rule = grammar.production_rule(*rule);
      let extract_vars = stack_states_for_rule(state_id, rule, grammar_info, state_map);
      quote! {
        #extract_vars
        Ok(::parser_generator::parser_state::ParserControl::Continue)
      }
    }
    Action::Accept => {
      quote! {
        let #enum_name(result) = state.accept() else { unsafe { ::std::hint::unreachable_unchecked() } };
        Ok(::parser_generator::parser_state::ParserControl::Accept(result))
      }
    }
  })
}

pub fn generate_state_action_function(
  state_id: StateId,
  grammar: &IndexedGrammar<String, ProductionRefName>,
  lr_table: &LRTable<String>,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap,
) -> TokenStreamResult {
  let token_type = grammar_info.terminal_type();
  let enum_name = enum_name(grammar_info);
  let fn_name = state_action_function_name(state_id);
  let result_type = root_production_type(grammar, grammar_info);

  let actions = lr_table
    .state_actions(state_id, grammar)
    .map(|(token, action)| {
      let matcher = token_matcher(&token)?;
      let apply_action = apply_action(&token, action, state_id, grammar, grammar_info, state_map)?;
      Ok(quote! {
        #matcher => { #apply_action }
      })
    })
    .try_collect_tokens()?;

  Ok(quote! {
    fn #fn_name<I, B: ::std::borrow::Borrow<#token_type>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, #enum_name, I>
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<#result_type>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state.stream().peek_next().map(::std::borrow::Borrow::borrow) {
        #actions
        _ => Err(::parser_generator::error::ParserError::new("Failed to parse"))
      }
    }
  })
}
