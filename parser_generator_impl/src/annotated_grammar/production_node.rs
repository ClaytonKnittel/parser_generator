use crate::{
  annotated_grammar::{
    production_ref::{ProductionRef, ProductionRefName},
    terminal::{Terminal, TerminalSymbol},
  },
  symbol::{Operator, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

pub enum ProductionNode {
  Production(ProductionRef),
  Terminal(Terminal),
}

impl ProductionNode {
  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let next_token = stream.expect_symbol()?;
    match next_token.symbol_type() {
      SymbolT::Op(Operator::BeginProd) => Ok(Self::Production(ProductionRef::parse(stream)?)),
      _ => Ok(Self::Terminal(Terminal::parse(stream)?)),
    }
  }
}

impl From<ProductionNode> for lr_table::grammar::ProductionNode<TerminalSymbol, ProductionRefName> {
  fn from(value: ProductionNode) -> Self {
    match value {
      ProductionNode::Production(production) => Self::Production(production.name().clone()),
      ProductionNode::Terminal(terminal) => Self::Terminal(terminal.into()),
    }
  }
}
