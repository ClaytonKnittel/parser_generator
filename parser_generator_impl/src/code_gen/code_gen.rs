use lr_table::{indexed_grammar::IndexedGrammar, lr_table::LRTable};
use quote::quote;

use crate::ParserGeneratorResult;

pub fn generate_parser<T>(
  grammar: &IndexedGrammar<T>,
  lr_table: &LRTable<T>,
) -> ParserGeneratorResult<proc_macro2::TokenStream> {
  Ok(quote! {})
}
