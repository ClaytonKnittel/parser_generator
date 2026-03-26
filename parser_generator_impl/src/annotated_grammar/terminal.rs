use crate::{
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TerminalSymbol {
  Symbol(String),
  Epsilon,
}

impl From<TerminalSymbol> for lr_table::vocabulary::AugmentedVocabToken<String> {
  fn from(value: TerminalSymbol) -> Self {
    match value {
      TerminalSymbol::Symbol(symbol) => Self::Token(symbol),
      TerminalSymbol::Epsilon => Self::Epsilon,
    }
  }
}

pub struct Terminal {
  symbol: TerminalSymbol,
  meta: SymbolMeta,
}

impl Terminal {
  fn new_symbol(text: String, meta: SymbolMeta) -> Self {
    Self {
      symbol: TerminalSymbol::Symbol(text),
      meta,
    }
  }

  fn new_epsilon(meta: SymbolMeta) -> Self {
    Self {
      symbol: TerminalSymbol::Epsilon,
      meta,
    }
  }

  pub fn symbol(&self) -> &TerminalSymbol {
    &self.symbol
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let next_token = stream.expect_symbol()?;
    match next_token.symbol_type() {
      SymbolT::Literal(text) => Ok(Self::new_symbol(text.to_owned(), next_token.take_meta())),
      SymbolT::Ident(ident) => Ok(Self::new_symbol(ident.to_owned(), next_token.take_meta())),
      SymbolT::Op(Operator::Bang) => Ok(Self::new_epsilon(next_token.take_meta())),
      _ => Err(next_token.meta().make_err("Unexpected token")),
    }
  }
}
