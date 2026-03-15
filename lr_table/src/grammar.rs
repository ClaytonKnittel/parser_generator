#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terminal<T> {
  EndOfStream,
  Epsilon,
  Symbol(T),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductionNode<T, L> {
  Terminal(Terminal<T>),
  Production(L),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionRule<T, L> {
  symbol: L,
  rule: Vec<ProductionNode<T, L>>,
}

impl<T, L> ProductionRule<T, L> {
  pub fn new(symbol: L, rule: Vec<ProductionNode<T, L>>) -> Self {
    Self { symbol, rule }
  }

  pub fn symbol(&self) -> &L {
    &self.symbol
  }

  pub fn rule(&self) -> &[ProductionNode<T, L>] {
    &self.rule
  }
}

pub struct Grammar<T, L> {
  productions: Vec<ProductionRule<T, L>>,
}

impl<T, L> Grammar<T, L> {
  pub fn new(productions: Vec<ProductionRule<T, L>>) -> Self {
    Self { productions }
  }

  pub fn productions(&self) -> &[ProductionRule<T, L>] {
    &self.productions
  }
}
