use crate::{
  error::InterceptResult,
  ident::Ident,
  symbol::{Operator, SymbolT},
  symbol_stream::SymbolStream,
  type_symbol::Type,
  Grammar, ParserGeneratorResult,
};

fn expect_symbol_with<F>(
  stream: &mut impl SymbolStream,
  cmp: F,
  message: impl Into<String>,
) -> ParserGeneratorResult
where
  F: FnOnce(&SymbolT) -> bool,
{
  let sym = stream.expect_symbol()?;
  if cmp(sym.symbol_type()) {
    Ok(())
  } else {
    Err(sym.meta().make_err(message))
  }
}

/// Parses a line of the form "option_name: value;"
fn parse_option<I, T, F>(
  stream: &mut I,
  option_name: &str,
  value_parser: F,
) -> ParserGeneratorResult<T>
where
  I: SymbolStream,
  F: FnOnce(&mut I) -> ParserGeneratorResult<T>,
{
  expect_symbol_with(
    stream,
    |sym| sym.is_identifier_with_name("terminal"),
    format!("Expected \"{option_name}\" keyword"),
  )?;
  expect_symbol_with(
    stream,
    |sym| sym.is_op(Operator::Colon),
    format!("Expected \":\" to follow \"{option_name}\" keyword."),
  )?;

  let value = value_parser(stream)?;

  expect_symbol_with(
    stream,
    |sym| sym.is_op(Operator::Semicolon),
    format!("Expected \";\" to follow \"{option_name}\" value."),
  )?;

  Ok(value)
}

struct GrammarInfo {
  name: Ident,
  terminal_type: Type,
  production_rules: Vec<i32>,
}

fn parse_name(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Ident> {
  parse_option(stream, "name", Ident::parse)
    .intercept("\"name: `MyName`;\" specifies the name the generated parser struct will be given")
}

fn parse_terminal_symbol_type(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Type> {
  parse_option(stream, "terminal", Type::parse)
    .intercept("\"terminal: `type`;\" specifies the `type` of tokens that this grammar will parse")
}

pub fn parse_grammar(mut stream: impl SymbolStream) -> ParserGeneratorResult<Grammar> {
  let name = parse_name(&mut stream)?;
  let terminal_type = parse_terminal_symbol_type(&mut stream)?;

  let grammar_info = GrammarInfo {
    name,
    terminal_type,
    production_rules: Vec::new(),
  };

  todo!();
}
