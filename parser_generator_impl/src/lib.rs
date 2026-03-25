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

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use crate::{
  annotated_grammar::parse_grammar::parse_grammar,
  code_gen::code_gen::generate_parser,
  error::{ParserGeneratorError, ParserGeneratorResult},
  symbol::tokenize_from_stream,
  symbol_stream::SymbolStreamImpl,
};

fn build_grammar(tokens: TokenStream) -> ParserGeneratorResult<TokenStream> {
  let list = tokenize_from_stream(tokens);
  let grammar_info = parse_grammar(SymbolStreamImpl::new(list))?;
  generate_parser(&grammar_info).map(TokenStream::from)
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
