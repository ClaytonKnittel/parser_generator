use std::{
  collections::{HashMap, hash_map::Entry},
  hash::Hash,
};

use crate::grammar::{Grammar, ProductionRule};

/// Each production label is given a unique ID densely packed starting from 0.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProductionLabel(usize);

struct RuleRange {
  start_index: usize,
  end_index: usize,
}

pub struct IndexedGrammar<T> {
  rules: Vec<ProductionRule<T, ProductionLabel>>,
  /// A map from `ProductionLabel` -> `RuleRange`
  rule_offset_map: Vec<RuleRange>,
}

impl<T> IndexedGrammar<T> {
  pub fn build<L: Clone + Eq + Hash>(grammar: &Grammar<T, L>) -> Self {
    let mut label_map = HashMap::new();
    let mut label_counts = Vec::new();

    for production in grammar.productions() {
      let map_len = label_map.len();
      let label = *label_map
        .entry(production.symbol().clone())
        .or_insert(ProductionLabel(map_len));

      if label_map.len() != map_len {
        debug_assert_eq!(label.0, label_counts.len());
        label_counts.push(1);
      } else {
        label_counts[label.0] += 1;
      }

      debug_assert_eq!(label_counts.len(), label_map.len());
    }

    todo!();
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    grammar::{Grammar, ProductionNode, ProductionRule, Terminal},
    indexed_grammar::IndexedGrammar,
  };

  #[gtest]
  fn test_one_rule() {
    let grammar = Grammar::new(vec![ProductionRule::new(
      'A',
      vec![ProductionNode::Terminal(Terminal::Symbol('a'))],
    )]);

    let indexed_grammar = IndexedGrammar::build(&grammar);
  }
}
