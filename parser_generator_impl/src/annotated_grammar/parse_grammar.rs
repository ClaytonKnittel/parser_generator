use std::str::FromStr;

use lr_table::{
  grammar::{Grammar, ProductionRuleIndex},
  indexed_grammar::IndexedGrammar,
  lr_state_map::LRStateMap,
  lr_table::LRTable,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{
  annotated_grammar::{
    label_type_map::LabelTypeMap, production_ref::ProductionRefName,
    production_rule::ProductionRule, util::expect_symbol_with,
  },
  error::InterceptResult,
  ident::Ident,
  symbol::Operator,
  symbol_stream::SymbolStream,
  type_symbol::Type,
  ParserGeneratorError, ParserGeneratorResult,
};

/// Parses a line of the form "option_name: value;" after option_name
fn parse_option_remainder<I, T, F>(
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
  parse_option_remainder(stream, option_name, value_parser)
}

pub enum TerminalType {
  Raw(Type),
  Enum(Type),
}

impl TerminalType {
  pub fn inner_type(&self) -> &Type {
    match self {
      Self::Raw(ty) | Self::Enum(ty) => ty,
    }
  }

  pub fn try_build_token(&self, text: &str) -> ParserGeneratorResult<TokenStream> {
    Ok(match self {
      Self::Raw(_) => {
        let literal = proc_macro2::Literal::from_str(text)
          .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;
        quote! { #literal }
      }
      Self::Enum(enum_type) => {
        let ident = proc_macro2::Ident::new(text, Span::call_site());
        quote! { #enum_type::#ident }
      }
    })
  }
}

pub struct GrammarInfo {
  name: Ident,
  terminal_type: TerminalType,
  label_types: LabelTypeMap,
  production_rules: Vec<ProductionRule>,
  indexed_grammar: IndexedGrammar<String, ProductionRefName>,
  lr_table: LRTable<String>,
}

impl GrammarInfo {
  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn terminal_type(&self) -> &TerminalType {
    &self.terminal_type
  }

  pub fn label_type(&self, label: &ProductionRefName) -> Option<&Type> {
    self.label_types.get(label)
  }

  pub fn production_rule(&self, index: ProductionRuleIndex) -> &ProductionRule {
    &self.production_rules[index.0]
  }

  pub fn grammar(&self) -> &IndexedGrammar<String, ProductionRefName> {
    &self.indexed_grammar
  }

  pub fn lr_table(&self) -> &LRTable<String> {
    &self.lr_table
  }

  pub fn build_lr_state_map(&self) -> ParserGeneratorResult<LRStateMap<'_>> {
    LRStateMap::build_from_lr_table(self.grammar(), self.lr_table())
      .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))
  }
}

fn parse_name(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Ident> {
  parse_option(stream, "name", Ident::parse)
    .intercept("\"name: `MyName`;\" specifies the name the generated parser struct will be given")
}

fn parse_terminal_symbol_type(
  stream: &mut impl SymbolStream,
) -> ParserGeneratorResult<TerminalType> {
  let next = stream
    .expect_symbol()
    .intercept("Expected \"terminal\" or \"enum_terminal\" option")?;

  if next.symbol_type().is_identifier_with_name("terminal") {
    parse_option_remainder(stream, "terminal", |stream| {
      Ok(TerminalType::Raw(Type::parse(stream)?))
    })
    .intercept("\"terminal: `type`;\" specifies the `type` of tokens that this grammar will parse")
  } else if next.symbol_type().is_identifier_with_name("enum_terminal") {
    parse_option_remainder(stream, "enum_terminal", |stream| {
      Ok(TerminalType::Enum(Type::parse(stream)?))
    })
    .intercept(
      "\"enum_terminal: `enum_type`;\" specifies the `enum_type` of tokens \
       that this grammar will parse",
    )
  } else {
    Err(
      next
        .meta()
        .make_err("Expected \"terminal\" or \"enum_terminal\" option"),
    )
  }
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

  let indexed_grammar = IndexedGrammar::build(&grammar)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;
  let lr_table = LRTable::build(&indexed_grammar)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;

  Ok(GrammarInfo {
    name,
    terminal_type,
    label_types,
    production_rules,
    indexed_grammar,
    lr_table,
  })
}
