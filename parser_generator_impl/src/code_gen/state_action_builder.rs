use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_table::{LRTable, StateId},
};
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::{states_enum::enum_name, util::TokenStreamResult},
};

pub fn state_action_function_name(state_id: StateId) -> syn::Ident {
  syn::Ident::new(
    &format!("parse_s{}", state_id.id()),
    proc_macro2::Span::call_site(),
  )
}

pub fn generate_state_action_function(
  state_id: StateId,
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let token_type = grammar_info.terminal_type();
  let enum_name = enum_name(grammar_info);
  let fn_name = state_action_function_name(state_id);

  Ok(quote! {
    fn #fn_name<I, B: ::std::borrow::Borrow<#token_type>>(
      stream: &mut ::parser_generator::parser_state::ParserState<B, #enum_name, I>
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl,
    > {
      Ok(::parser_generator::parser_state::ParserControl::Continue)
    }
  })
}
