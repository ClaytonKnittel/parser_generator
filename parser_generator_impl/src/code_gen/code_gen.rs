use lr_table::{indexed_grammar::IndexedGrammar, lr_state_map::LRStateMap, lr_table::LRTable};
use proc_macro::Span;
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::{
    collect_tokens::TryCollectTokens, parse_loop::generate_parse_loop,
    state_action_builder::generate_state_action_function, states_enum::generate_dfa_states,
    util::TokenStreamResult,
  },
  ParserGeneratorError,
};

fn root_production_type(
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  grammar_info: &GrammarInfo,
) -> proc_macro2::TokenStream {
  let root_label = grammar.orig_production_label(grammar.root_production_label());
  match grammar_info.label_type(root_label) {
    Some(root_type) => quote! { #root_type },
    None => quote! { () },
  }
}

pub fn generate_parser(
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let grammar_name = grammar_info.name().make_syn_ident();
  let token_type = grammar_info.terminal_type();

  let result_type = root_production_type(grammar, grammar_info);

  let dfa_states_enum = generate_dfa_states(grammar, lr_table, grammar_info)?;

  let action_functions = lr_table
    .states()
    .map(|state_id| generate_state_action_function(state_id, grammar, lr_table, grammar_info))
    .try_collect_tokens()?;

  let parse_loop = generate_parse_loop(lr_table, grammar_info)?;

  Ok(quote! {
    struct #grammar_name;
    impl ::parser_generator::parser::Parser for #grammar_name {
      type Token = #token_type;
      type Value = #result_type;

      fn parse<I, B>(
        input_stream: I
      ) -> ::parser_generator::error::ParserResult<Self::Value>
      where
        I: IntoIterator<Item = B>,
        B: ::std::borrow::Borrow<#token_type>,
      {
        #dfa_states_enum
        #action_functions
        #parse_loop
      }
    }
  })
}
