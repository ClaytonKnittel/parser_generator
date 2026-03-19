#![feature(get_mut_unchecked, proc_macro_span)]

extern crate proc_macro;
mod code_gen;
mod error;
mod lr_table_builder;
mod production;
mod symbol;
mod util;

use lr_table_builder::LRTable;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use production::Grammar;

use crate::{error::ParserGeneratorResult, symbol::tokenize_from_stream};

fn build_grammar(tokens: TokenStream) -> ParserGeneratorResult<TokenStream> {
  let list = tokenize_from_stream(tokens)?;
  let grammar = Grammar::from(list);
  let lr_table = LRTable::from_grammar(&grammar).unwrap_or_else(|err| err.raise());
  let syn_tree = code_gen::to_match_loop(&grammar, &lr_table).unwrap_or_else(|err| err.raise());
  Ok(syn_tree.into())
}

#[proc_macro_error]
#[proc_macro]
/// Constructs an LR(1) parser based on the definition provided.
pub fn grammar(tokens: TokenStream) -> TokenStream {
  match build_grammar(tokens) {
    Ok(tokens) => tokens,
    Err(err) => err.abort(),
  }
}
