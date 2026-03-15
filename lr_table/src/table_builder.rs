use crate::{
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, IndexedProductionRule},
};

#[derive(Debug, PartialEq, Eq)]
struct ProductionPosition<'a, T> {
  production: &'a IndexedProductionRule<T>,
  position: usize,
}

impl<'a, T> Clone for ProductionPosition<'a, T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<'a, T> Copy for ProductionPosition<'a, T> {}

fn closure<'a, T>(
  position: ProductionPosition<'a, T>,
  grammar: &'a IndexedGrammar<T>,
) -> impl Iterator<Item = ProductionPosition<'a, T>> {
  let mut positions = Vec::new();
  let mut stack = vec![position];
  while let Some(pos) = stack.pop() {
    positions.push(pos.clone());
    if pos.position >= pos.production.rule().len() {
      continue;
    }

    if let ProductionNode::Production(label) = &pos.production.rule()[pos.position] {
      for production in grammar.productions_for_label(*label) {
        stack.push(ProductionPosition {
          production,
          position: 0,
        });
      }
    }
  }

  positions.into_iter()
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::{
    grammar::{Grammar, ProductionNode, ProductionRule, Terminal},
    indexed_grammar::{IndexedGrammar, IndexedProductionRule, ProductionLabel},
    table_builder::{ProductionPosition, closure},
  };

  #[gtest]
  fn test_small_closure() {
    let grammar = Grammar::new(vec![
      ProductionRule::new('A', vec![ProductionNode::Production('B')]),
      ProductionRule::new('B', vec![ProductionNode::Terminal(Terminal::Symbol('a'))]),
    ]);

    let indexed = IndexedGrammar::build(&grammar);
    let production = &indexed.productions_for_label(ProductionLabel::new(0))[0];
    expect_that!(
      closure(
        ProductionPosition {
          production,
          position: 0
        },
        &indexed
      )
      .collect_vec(),
      unordered_elements_are![
        &ProductionPosition {
          production: &IndexedProductionRule::new(vec![ProductionNode::Production(
            ProductionLabel::new(1)
          )]),
          position: 0
        },
        &ProductionPosition {
          production: &IndexedProductionRule::new(vec![ProductionNode::Terminal(
            Terminal::Symbol('a')
          )]),
          position: 0
        }
      ]
    );
  }
}
