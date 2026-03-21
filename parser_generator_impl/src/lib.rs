#![feature(proc_macro_span)]

extern crate proc_macro;
mod annotated_grammar;
mod code_gen;
mod error;
mod ident;
mod symbol;
mod symbol_stream;
mod type_symbol;
mod util;

use lr_table::{indexed_grammar::IndexedGrammar, lr_table::LRTable};
use proc_macro::{Span, TokenStream};
use proc_macro_error::proc_macro_error;

use crate::{
  annotated_grammar::parse_grammar::parse_grammar,
  error::{ParserGeneratorError, ParserGeneratorResult},
  symbol::tokenize_from_stream,
  symbol_stream::SymbolStreamImpl,
};

fn build_grammar(tokens: TokenStream) -> ParserGeneratorResult<TokenStream> {
  let list = tokenize_from_stream(tokens);
  let grammar_info = parse_grammar(SymbolStreamImpl::new(list))?;
  let grammar = grammar_info.build_lr_table_grammar();
  let grammar = IndexedGrammar::build(&grammar)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;
  let lr_table = LRTable::build(&grammar)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;
  todo!();
  // let syn_tree = code_gen::to_match_loop(&grammar, &lr_table).unwrap_or_else(|err| err.raise());
  // Ok(syn_tree.into())
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
