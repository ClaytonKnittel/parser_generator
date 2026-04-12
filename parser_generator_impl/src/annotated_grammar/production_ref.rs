use std::fmt::Debug;

use crate::{
  ParserGeneratorResult,
  annotated_grammar::util::expect_symbol_with,
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProductionRefName(pub String);

impl ProductionRefName {
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

#[derive(Clone)]
pub struct ProductionRef {
  name: ProductionRefName,
  meta: SymbolMeta,
}

impl ProductionRef {
  pub fn name(&self) -> &ProductionRefName {
    &self.name
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let mut meta = expect_symbol_with(
      stream,
      |sym| sym.is_op(Operator::BeginProd),
      "Expected production name to begin with '<'.",
    )?;

    let prod_name_sym = stream.expect_symbol()?;
    let name = match prod_name_sym.symbol_type() {
      SymbolT::Ident(ident) => Ok(ProductionRefName(ident.to_owned())),
      _ => return Err(prod_name_sym.meta().make_err("Expected production name.")),
    }?;

    let end_meta = expect_symbol_with(
      stream,
      |sym| sym.is_op(Operator::EndProd),
      "Expected production name to end with '>'.",
    )?;
    meta.merge(&end_meta)?;

    Ok(Self { name, meta })
  }
}

impl Debug for ProductionRefName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<{}>", self.0)
  }
}
