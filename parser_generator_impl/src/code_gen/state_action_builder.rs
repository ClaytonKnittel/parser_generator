use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_table::{LRTable, StateId},
};
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::util::TokenStreamResult,
};

pub fn generate_state_action_function(
  state_id: StateId,
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let token_type = grammar_info.terminal_type();
  let fn_name = syn::Ident::new(
    &format!("parse_s{}", state_id.id()),
    proc_macro2::Span::call_site(),
  );

  Ok(quote! {
    fn #fn_name<I>(
      stream: &mut ::parser_generator::parser_stream::ParserStream<#token_type, I>
    ) -> ::parser_generator::error::ParserResult {
      todo!();
    }
  })
}
