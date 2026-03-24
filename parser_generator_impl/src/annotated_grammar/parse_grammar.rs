use std::collections::HashMap;

use lr_table::grammar::{Grammar, ProductionRuleIndex};

use crate::{
  annotated_grammar::{
    label_type_map::LabelTypeMap, production_ref::ProductionRefName,
    production_rule::ProductionRule, terminal::TerminalSymbol, util::expect_symbol_with,
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
  label_types: LabelTypeMap,
  production_rules: Vec<ProductionRule>,
  grammar: Grammar<String, ProductionRefName>,
}

impl GrammarInfo {
  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn terminal_type(&self) -> &Type {
    &self.terminal_type
  }

  pub fn label_type(&self, label: &ProductionRefName) -> Option<&Type> {
    self.label_types.get(label)
  }

  pub fn production_rule(&self, index: ProductionRuleIndex) -> &ProductionRule {
    &self.production_rules[index.0]
  }

  pub fn lr_table_grammar(&self) -> &Grammar<String, ProductionRefName> {
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

  let mut label_types = LabelTypeMap::new();

  let mut production_rules = Vec::new();
  while stream.peek().is_some() {
    production_rules.extend(ProductionRule::parse(&mut stream, &mut label_types)?);
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
    label_types,
    production_rules,
    grammar,
  })
}
