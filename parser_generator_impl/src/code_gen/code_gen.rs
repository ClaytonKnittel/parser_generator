use lr_table::{indexed_grammar::IndexedGrammar, lr_table::LRTable};
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::{states_enum::generate_dfa_states, util::TokenStreamResult},
};

pub fn generate_parser(
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let grammar_name = grammar_info.name().make_syn_ident();
  let token_type = grammar_info.terminal_type();
  let result_type = quote! { () };

  let dfa_states_enum = generate_dfa_states(grammar, lr_table, grammar_info)?;

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
        B: std::borrow::Borrow<Self::Token>,
      {
        #dfa_states_enum

        todo!();
      }
    }
  })
}
