use std::iter::Peekable;

use proc_macro::{Span, TokenStream};

use crate::{symbol::Symbol, Grammar, ParserGeneratorResult};

struct TerminalType {
  pub tokens: TokenStream,
  pub span: Span,
}

fn parse_name(
  symbol_stream: &mut Peekable<impl Iterator<Item = ParserGeneratorResult<Symbol>>>,
) -> ParserGeneratorResult<String> {
  Ok("test".into())
}

fn parse_terminal_symbol_type(
  symbol_stream: &mut Peekable<impl Iterator<Item = ParserGeneratorResult<Symbol>>>,
) -> ParserGeneratorResult<String> {
  Ok("test".into())
}

pub fn parse_grammar(
  symbol_stream: impl Iterator<Item = ParserGeneratorResult<Symbol>>,
) -> ParserGeneratorResult<Grammar> {
  let mut symbol_stream = symbol_stream.peekable();

  let grammar_name = parse_name(&mut symbol_stream)?;
  let terminal_symbol_type = parse_terminal_symbol_type(&mut symbol_stream)?;

  todo!();
}
