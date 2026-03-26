use crate::{
  annotated_grammar::{
    production_ref::{ProductionRef, ProductionRefName},
    terminal::Terminal,
  },
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

pub enum ProductionNode {
  Production(ProductionRef),
  Terminal(Terminal),
}

impl ProductionNode {
  pub fn meta(&self) -> &SymbolMeta {
    match self {
      Self::Production(production) => production.meta(),
      Self::Terminal(terminal) => terminal.meta(),
    }
  }

  pub fn to_lr_node(&self) -> lr_table::grammar::ProductionNode<String, ProductionRefName> {
    match self {
      ProductionNode::Production(production) => {
        lr_table::grammar::ProductionNode::Production(production.name().clone())
      }
      ProductionNode::Terminal(terminal) => {
        lr_table::grammar::ProductionNode::Terminal(terminal.symbol().clone().into())
      }
    }
  }

  pub fn is_epsilon(&self) -> bool {
    match self {
      Self::Terminal(terminal) => terminal.symbol().is_epsilon(),
      Self::Production(_) => false,
    }
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let next_token = stream.peek_expect_symbol()?;
    match next_token.symbol_type() {
      SymbolT::Op(Operator::BeginProd) => Ok(Self::Production(ProductionRef::parse(stream)?)),
      _ => Ok(Self::Terminal(Terminal::parse(stream)?)),
    }
  }
}
