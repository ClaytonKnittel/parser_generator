use crate::error::{LRTableError, LRTableResult};

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

impl Grammar<u8, String> {
  pub fn from_grammar_str(grammar_str: &str) -> LRTableResult<Self> {
    Ok(Self::new(
      grammar_str
        .lines()
        .map(|line| -> LRTableResult<_> {
          let (production, rule) = line
            .split_once("->")
            .ok_or_else(|| LRTableError::new(format!("Line \"{line}\" missing \"->\"")))?;
          let production = production.trim();
          if !production.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(LRTableError::new(format!("Production label \"{production}\" is not all ASCII uppercase")).into());
          }

          Ok(ProductionRule::new(
            production.to_owned(),
            rule
              .trim()
              .split_ascii_whitespace()
              .map(|node| {
                let bytes = node.as_bytes();
                if node.chars().all(|c| c.is_ascii_uppercase()) {
                  Ok(ProductionNode::Production(node.to_owned()))
                } else if bytes.len() == 1 && bytes[0].is_ascii() {
                  Ok(ProductionNode::Terminal(Terminal::Symbol(bytes[0])))
                } else {
                  Err(LRTableError::new(format!(
                    "Node \"{node}\" is not all ASCII uppercase (production) or lowercase letter (terminal)"
                  )))
                }
              })
              .collect::<Result<_, _>>()?,
          ))
        })
        .collect::<Result<_, _>>()?,
    ))
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::grammar::{Grammar, ProductionNode, ProductionRule, Terminal};

  #[gtest]
  fn test_parse_from_str() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> c"#,
    )
    .unwrap();

    expect_that!(
      grammar.productions(),
      unordered_elements_are![
        &ProductionRule::new(
          "A".to_owned(),
          vec![ProductionNode::Production("B".to_owned())]
        ),
        &ProductionRule::new(
          "B".to_owned(),
          vec![ProductionNode::Terminal(Terminal::Symbol(b'c'))]
        )
      ]
    );
  }
}
