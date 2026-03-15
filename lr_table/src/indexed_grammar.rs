use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

use crate::grammar::{Grammar, ProductionNode};

/// Each production label is given a unique ID densely packed starting from 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionLabel(usize);

impl ProductionLabel {
  #[cfg(test)]
  pub(crate) fn new(id: usize) -> Self {
    Self(id)
  }

  pub fn id(&self) -> usize {
    self.0
  }
}

/// Each particular instance of a production rule is given a unique ID densely
/// packed starting from 0. This is just the index into
/// `IndexedGrammar::rules`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionRuleId(usize);

struct RuleRange {
  start_index: usize,
  end_index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedProductionRule<T> {
  rule: Vec<ProductionNode<T, ProductionLabel>>,
}

impl<T> IndexedProductionRule<T> {
  pub fn new(rule: Vec<ProductionNode<T, ProductionLabel>>) -> Self {
    Self { rule }
  }

  pub fn rule(&self) -> &[ProductionNode<T, ProductionLabel>] {
    &self.rule
  }
}

pub struct IndexedGrammar<T> {
  rules: Vec<IndexedProductionRule<T>>,
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
      .flat_map(|group| {
        group.iter().map(|production| {
          IndexedProductionRule::new(
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

impl<T> IndexedGrammar<T> {
  #[cfg(test)]
  fn labels_count(&self) -> usize {
    self.rule_offset_map.len()
  }

  /// Returns a range over the production rules for a particular production label.
  pub fn productions_for_label(
    &self,
    label: ProductionLabel,
  ) -> impl Iterator<Item = ProductionRuleId> {
    let range = &self.rule_offset_map[label.0];
    (range.start_index..range.end_index).map(ProductionRuleId)
  }

  pub fn production_rule(&self, id: ProductionRuleId) -> &IndexedProductionRule<T> {
    &self.rules[id.0]
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    grammar::{Grammar, ProductionNode, Terminal},
    indexed_grammar::{IndexedGrammar, IndexedProductionRule, ProductionLabel},
  };

  fn production_rules<T>(
    grammar: &IndexedGrammar<T>,
    label: ProductionLabel,
  ) -> Vec<&IndexedProductionRule<T>> {
    grammar
      .productions_for_label(label)
      .map(|id| grammar.production_rule(id))
      .collect()
  }

  #[gtest]
  fn test_one_rule() {
    let grammar = Grammar::from_grammar_str("A -> a").unwrap();

    let indexed_grammar = IndexedGrammar::build(&grammar);
    assert_eq!(indexed_grammar.labels_count(), 1);
    expect_that!(
      production_rules(&indexed_grammar, ProductionLabel(0)),
      elements_are![&&IndexedProductionRule::new(vec![
        ProductionNode::Terminal(Terminal::Symbol("a".to_owned()))
      ])]
    );
  }

  #[gtest]
  fn test_two_productions() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a
         B -> b"#,
    )
    .unwrap();

    let indexed_grammar = IndexedGrammar::build(&grammar);
    assert_eq!(indexed_grammar.labels_count(), 2);
    expect_that!(
      production_rules(&indexed_grammar, ProductionLabel(0)),
      elements_are![&&IndexedProductionRule::new(vec![
        ProductionNode::Terminal(Terminal::Symbol("a".to_owned()))
      ])]
    );
    expect_that!(
      production_rules(&indexed_grammar, ProductionLabel(1)),
      elements_are![&&IndexedProductionRule::new(vec![
        ProductionNode::Terminal(Terminal::Symbol("b".to_owned()))
      ])]
    );
  }

  #[gtest]
  fn test_two_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a
         A -> b"#,
    )
    .unwrap();

    let indexed_grammar = IndexedGrammar::build(&grammar);
    assert_eq!(indexed_grammar.labels_count(), 1);
    expect_that!(
      production_rules(&indexed_grammar, ProductionLabel(0)),
      elements_are![
        &&IndexedProductionRule::new(vec![ProductionNode::Terminal(Terminal::Symbol(
          "a".to_owned()
        ))]),
        &&IndexedProductionRule::new(vec![ProductionNode::Terminal(Terminal::Symbol(
          "b".to_owned()
        ))])
      ]
    );
  }
}
