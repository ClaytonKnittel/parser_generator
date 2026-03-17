use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData};

use itertools::Itertools;

use crate::{
  bit_set::BitSet,
  grammar::{Grammar, ProductionNode, ProductionRule},
};

/// Each production label is given a unique ID densely packed starting from 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionLabel(usize);

/// Each particular instance of a production rule is given a unique ID densely
/// packed starting from 0. This is just the index into
/// `IndexedGrammar::rules`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProductionRuleId(usize);

impl Debug for ProductionRuleId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RuleId({})", self.0)
  }
}

trait Label: Copy {
  fn id(self) -> usize;
  fn from_id(id: usize) -> Self;
}

impl Label for ProductionLabel {
  fn id(self) -> usize {
    self.0
  }
  fn from_id(id: usize) -> Self {
    Self(id)
  }
}

impl Label for ProductionRuleId {
  fn id(self) -> usize {
    self.0
  }
  fn from_id(id: usize) -> Self {
    Self(id)
  }
}

pub struct FixedSizeSet<L> {
  set: BitSet,
  _phantom: PhantomData<L>,
}

impl<L: Label> FixedSizeSet<L> {
  fn new(capacity: usize) -> Self {
    Self {
      set: BitSet::new(capacity),
      _phantom: PhantomData,
    }
  }

  pub fn get(&self, label: L) -> bool {
    self.set.get(label.id())
  }

  pub fn set(&mut self, label: L) {
    self.set.set(label.id());
  }
}

pub struct FixedSizeMap<L, T> {
  map: Vec<T>,
  _phantom: PhantomData<L>,
}

impl<L: Label, T: Default> FixedSizeMap<L, T> {
  fn new(capacity: usize) -> Self {
    Self {
      map: (0..capacity).map(|_| T::default()).collect(),
      _phantom: PhantomData,
    }
  }
}

impl<L: Label, T> FixedSizeMap<L, T> {
  pub fn get(&self, label: L) -> &T {
    &self.map[label.id()]
  }

  pub fn get_mut(&mut self, label: L) -> &mut T {
    &mut self.map[label.id()]
  }
}

impl<L: Debug + Label, T: Debug> Debug for FixedSizeMap<L, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..self.map.len())
        .map(L::from_id)
        .map(|label| { format!("{label:?}: {:?}", self.get(label)) })
        .join(", ")
    )
  }
}

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
  fn build_from_grammar<L: Clone + Eq + Hash>(
    grammar: &Grammar<T, L>,
  ) -> (Self, HashMap<L, ProductionLabel>) {
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
      .flat_map(|(index, group)| {
        let label = ProductionLabel(index);
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

    (
      Self {
        rules,
        rule_offset_map,
      },
      label_map,
    )
  }

  #[cfg(test)]
  pub fn build_with_label_map<L: Clone + Eq + Hash>(
    grammar: &Grammar<T, L>,
  ) -> (Self, HashMap<L, ProductionLabel>) {
    Self::build_from_grammar(grammar)
  }

  pub fn build<L: Clone + Eq + Hash>(grammar: &Grammar<T, L>) -> Self {
    Self::build_from_grammar(grammar).0
  }
}

impl<T> IndexedGrammar<T> {
  pub fn root_production_label(&self) -> ProductionLabel {
    ProductionLabel(0)
  }

  pub fn all_production_labels(&self) -> impl Iterator<Item = ProductionLabel> {
    (0..self.labels_count()).map(ProductionLabel)
  }

  fn labels_count(&self) -> usize {
    self.rule_offset_map.len()
  }

  pub fn new_production_rule_set(&self) -> FixedSizeSet<ProductionRuleId> {
    FixedSizeSet::new(self.rules.len())
  }

  pub fn new_production_label_map<U: Default>(&self) -> FixedSizeMap<ProductionLabel, U> {
    FixedSizeMap::new(self.labels_count())
  }

  pub fn new_production_rule_map<U: Default>(&self) -> FixedSizeMap<ProductionRuleId, U> {
    FixedSizeMap::new(self.rules.len())
  }

  /// Returns a range over the production rules for a particular production label.
  pub fn productions_for_label(
    &self,
    label: ProductionLabel,
  ) -> impl Iterator<Item = ProductionRuleId> {
    let range = &self.rule_offset_map[label.0];
    (range.start_index..range.end_index).map(ProductionRuleId)
  }

  pub fn production_rule(&self, id: ProductionRuleId) -> &ProductionRule<T, ProductionLabel> {
    &self.rules[id.0]
  }

  pub fn rule_label(&self, id: ProductionRuleId) -> ProductionLabel {
    *self.production_rule(id).symbol()
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    grammar::{Grammar, ProductionNode, ProductionRule},
    indexed_grammar::{IndexedGrammar, ProductionLabel},
    vocabulary::AugmentedVocab,
  };

  fn production_rules<T>(
    grammar: &IndexedGrammar<T>,
    label: ProductionLabel,
  ) -> Vec<&ProductionRule<T, ProductionLabel>> {
    grammar
      .productions_for_label(label)
      .map(|id| grammar.production_rule(id))
      .collect()
  }

  #[gtest]
  fn test_one_rule() {
    let grammar = Grammar::from_grammar_str("A -> a").unwrap();

    let (indexed_grammar, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    assert_eq!(indexed_grammar.labels_count(), 1);
    let label_a = *label_map.get("A").unwrap();
    expect_that!(
      production_rules(&indexed_grammar, label_a),
      elements_are![&&ProductionRule::new(
        label_a,
        vec![ProductionNode::Terminal(AugmentedVocab::Token(b'a'))]
      )]
    );
  }

  #[gtest]
  fn test_two_productions() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a
         B -> b"#,
    )
    .unwrap();

    let (indexed_grammar, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    assert_eq!(indexed_grammar.labels_count(), 2);
    let label_a = *label_map.get("A").unwrap();
    expect_that!(
      production_rules(&indexed_grammar, label_a),
      elements_are![&&ProductionRule::new(
        label_a,
        vec![ProductionNode::Terminal(AugmentedVocab::Token(b'a'))]
      )]
    );
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      production_rules(&indexed_grammar, label_b),
      elements_are![&&ProductionRule::new(
        label_b,
        vec![ProductionNode::Terminal(AugmentedVocab::Token(b'b'))]
      )]
    );
  }

  #[gtest]
  fn test_two_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a
         A -> b"#,
    )
    .unwrap();

    let (indexed_grammar, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    assert_eq!(indexed_grammar.labels_count(), 1);
    let label_a = *label_map.get("A").unwrap();
    expect_that!(
      production_rules(&indexed_grammar, label_a),
      elements_are![
        &&ProductionRule::new(
          label_a,
          vec![ProductionNode::Terminal(AugmentedVocab::Token(b'a'))]
        ),
        &&ProductionRule::new(
          label_a,
          vec![ProductionNode::Terminal(AugmentedVocab::Token(b'b'))]
        )
      ]
    );
  }
}
