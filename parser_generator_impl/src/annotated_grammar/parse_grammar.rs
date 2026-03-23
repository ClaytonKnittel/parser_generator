use lr_table::grammar::{Grammar, ProductionRuleIndex};

use crate::{
  annotated_grammar::{
    production_ref::ProductionRefName,
    production_rule::{Constructor, ProductionRule},
    terminal::TerminalSymbol,
    util::expect_symbol_with,
  },
  error::InterceptResult,
  ident::Ident,
  symbol::Operator,
  symbol_stream::SymbolStream,
  type_symbol::Type,
  ParserGeneratorResult,
};

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
    |sym| sym.is_identifier_with_name(option_name),
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

pub struct GrammarInfo {
  name: Ident,
  terminal_type: Type,
  production_rule_constructors: Vec<Option<Constructor>>,
  grammar: Grammar<TerminalSymbol, ProductionRefName>,
}

impl GrammarInfo {
  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn terminal_type(&self) -> &Type {
    &self.terminal_type
  }

  pub fn constructor(&self, index: ProductionRuleIndex) -> Option<&Constructor> {
    self.production_rule_constructors[index.0].as_ref()
  }

  pub fn lr_table_grammar(&self) -> &Grammar<TerminalSymbol, ProductionRefName> {
    &self.grammar
  }
}

fn parse_name(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Ident> {
  parse_option(stream, "name", Ident::parse)
    .intercept("\"name: `MyName`;\" specifies the name the generated parser struct will be given")
}

fn parse_terminal_symbol_type(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Type> {
  parse_option(stream, "terminal", Type::parse)
    .intercept("\"terminal: `type`;\" specifies the `type` of tokens that this grammar will parse")
}

pub fn parse_grammar(mut stream: impl SymbolStream) -> ParserGeneratorResult<GrammarInfo> {
  let name = parse_name(&mut stream)?;
  let terminal_type = parse_terminal_symbol_type(&mut stream)?;

  let mut production_rules = Vec::new();
  while stream.peek().is_some() {
    production_rules.extend(ProductionRule::parse(&mut stream)?);
  }

  // TODO: build two objects in parallel, this grammar and one with just
  // metadata / codegen-related stuff.
  let grammar = Grammar::new(
    production_rules
      .iter()
      .map(ProductionRule::to_lr_production_rule)
      .collect(),
  );

  Ok(GrammarInfo {
    name,
    terminal_type,
    production_rules,
    grammar,
  })
}
