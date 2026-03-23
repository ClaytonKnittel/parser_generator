use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_table::{LRTable, StateId},
};
use quote::quote;

use crate::{
  annotated_grammar::parse_grammar::GrammarInfo, type_symbol::Type, ParserGeneratorResult,
};

type TokenStreamResult = ParserGeneratorResult<proc_macro2::TokenStream>;

fn generate_enum_variant(state: StateId, return_type: Option<Type>) -> proc_macro2::TokenStream {
  todo!();
}

fn generate_dfa_states<T>(
  grammar: &IndexedGrammar<T>,
  lr_table: &LRTable<T>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let grammar_name = grammar_info.name();
  let dfa_enum_name = syn::Ident::new(
    &format!("{}DfaStates", grammar_name.name()),
    grammar_name.meta().span2(),
  );

  lr_table.states();

  Ok(quote! {
    struct #dfa_enum_name {
    }
  })
}

pub fn generate_parser<T>(
  grammar: &IndexedGrammar<T>,
  lr_table: &LRTable<T>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  Ok(quote! {})
}
