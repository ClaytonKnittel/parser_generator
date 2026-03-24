use std::str::FromStr;

use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_table::{Action, LRTable, StateId},
  vocabulary::AugmentedVocabToken,
};
use quote::quote;

use crate::{
  annotated_grammar::{parse_grammar::GrammarInfo, production_ref::ProductionRefName},
  code_gen::{
    collect_tokens::TryCollectTokens,
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

fn apply_action(
  token: &AugmentedVocabToken<String>,
  action: &Action,
  state_id: StateId,
  grammar: &IndexedGrammar<String, ProductionRefName>,
  grammar_info: &GrammarInfo,
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
      quote! {
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
) -> TokenStreamResult {
  let token_type = grammar_info.terminal_type();
  let enum_name = enum_name(grammar_info);
  let fn_name = state_action_function_name(state_id);
  let result_type = root_production_type(grammar, grammar_info);

  let actions = lr_table
    .state_actions(state_id, grammar)
    .map(|(token, action)| {
      let matcher = token_matcher(&token)?;
      let apply_action = apply_action(&token, action, state_id, grammar, grammar_info)?;
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
