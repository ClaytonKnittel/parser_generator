use std::{
  collections::{HashMap, hash_map::Entry},
  hash::Hash,
};

use itertools::Itertools;

use crate::grammar::{Grammar, ProductionNode, ProductionRule};

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

impl<T: Clone> IndexedGrammar<T> {
  pub fn build<L: Clone + Eq + Hash>(grammar: &Grammar<T, L>) -> Self {
    let mut label_map = HashMap::new();
    let mut label_groups = Vec::new();

    for production in grammar.productions() {
      let map_len = label_map.len();
      let label = *label_map
        .entry(production.symbol().clone())
        .or_insert(ProductionLabel(map_len));

      if label_map.len() != map_len {
        debug_assert_eq!(label.0, label_groups.len());
        label_groups.push(vec![production]);
      } else {
        label_groups[label.0].push(production);
      }

      debug_assert_eq!(label_groups.len(), label_groups.len());
    }

    let rules = label_groups
      .iter()
      .enumerate()
      .flat_map(|(i, group)| {
        let label = ProductionLabel(i);
        let label_map = &label_map;
        group.iter().map(move |production| {
          ProductionRule::new(
            label,
            production
              .rule()
              .iter()
              .map(|node| match node {
                ProductionNode::Production(user_label) => {
                  ProductionNode::Production(*label_map.get(user_label).unwrap())
                }
                ProductionNode::Terminal(terminal) => ProductionNode::Terminal(terminal.clone()),
              })
              .collect(),
          )
        })
      })
      .collect_vec();

    let rule_offset_map = label_groups
      .iter()
      .scan(0, |total, group| {
        let start_index = *total;
        let end_index = *total + group.len();
        *total = end_index;
        Some(RuleRange {
          start_index,
          end_index,
        })
      })
      .collect_vec();

    Self {
      rules,
      rule_offset_map,
    }
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
