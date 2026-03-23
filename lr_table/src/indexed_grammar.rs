use std::{collections::HashMap, fmt::Debug, hash::Hash};

use itertools::Itertools;

use crate::{
  error::{LRTableResult, grammar_error},
  fixed_map::{FixedSizeMap, FixedSizeSet, Label, SparseFixedSizeMap},
  grammar::{Grammar, ProductionNode, ProductionRule, ProductionRuleIndex},
  vocabulary::{AugmentedTokenId, AugmentedVocab, AugmentedVocabToken, TokenId, VocabularyBuilder},
};

/// Each production label is given a unique ID densely packed starting from 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionLabel(usize);

/// Each particular instance of a production rule is given a unique ID densely
/// packed starting from 0. This is just the index into
/// `IndexedGrammar::rules`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductionRuleId(usize);

impl Debug for ProductionRuleId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RuleId({})", self.0)
  }
}

impl Label for ProductionLabel {
  fn id(&self) -> usize {
    self.0
  }
  fn from_id(id: usize) -> Self {
    Self(id)
  }
}

impl Label for ProductionRuleId {
  fn id(&self) -> usize {
    self.0
  }
  fn from_id(id: usize) -> Self {
    Self(id)
  }
}

pub type IndexedProductionNode = ProductionNode<TokenId, ProductionLabel>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedProductionRule {
  rule: ProductionRule<TokenId, ProductionLabel>,
  original_index: ProductionRuleIndex,
}

impl IndexedProductionRule {
  fn new(
    symbol: ProductionLabel,
    rule: Vec<ProductionNode<TokenId, ProductionLabel>>,
    original_index: ProductionRuleIndex,
  ) -> Self {
    Self {
      rule: ProductionRule::new(symbol, rule),
      original_index,
    }
  }

  pub fn symbol(&self) -> &ProductionLabel {
    self.rule.symbol()
  }

  pub fn rule(&self) -> &[ProductionNode<TokenId, ProductionLabel>] {
    self.rule.rule()
  }

  pub fn original_index(&self) -> ProductionRuleIndex {
    self.original_index
  }

  #[cfg(debug_assertions)]
  pub fn rules_excluding_epsilon(
    &self,
  ) -> impl Iterator<Item = &ProductionNode<TokenId, ProductionLabel>> {
    self.rule.rules_excluding_epsilon()
  }
}

/// The key for closure partitions. This is either a terminal, a production
/// label, or `None` (special case for positions at the end of their rules).
pub type NextTokenCategory = Option<IndexedProductionNode>;

pub struct SparsePartitionMap<V> {
  map: SparseFixedSizeMap<usize, V>,
  vocab_size: usize,
}

impl<V> SparsePartitionMap<V> {
  fn new(num_productions: usize, vocab_size: usize) -> Self {
    Self {
      map: SparseFixedSizeMap::new(num_productions + vocab_size + 1),
      vocab_size,
    }
  }

  fn static_id(label: &NextTokenCategory, vocab_size: usize) -> usize {
    match label {
      Some(ProductionNode::Production(label)) => vocab_size + 1 + label.id(),
      Some(ProductionNode::Terminal(terminal)) => terminal.id() + 1,
      None => 0,
    }
  }

  fn static_from_id(id: usize, vocab_size: usize) -> NextTokenCategory {
    if id == 0 {
      None
    } else if id <= vocab_size {
      Some(ProductionNode::Terminal(AugmentedVocabToken::from_id(
        id - 1,
      )))
    } else {
      Some(ProductionNode::Production(ProductionLabel::from_id(
        id - (vocab_size + 1),
      )))
    }
  }

  fn category_id(&self, label: &NextTokenCategory) -> usize {
    Self::static_id(label, self.vocab_size)
  }

  fn category_from_id(&self, id: usize) -> NextTokenCategory {
    Self::static_from_id(id, self.vocab_size)
  }

  pub fn get_mut_or_default(&mut self, label: &NextTokenCategory) -> &mut V
  where
    V: Default,
  {
    self.map.get_mut_or_default(&self.category_id(label))
  }

  pub fn iter(&self) -> impl Iterator<Item = (NextTokenCategory, &V)> {
    self
      .map
      .iter()
      .map(|(label_id, value)| (self.category_from_id(label_id), value))
  }
}

impl<V: Default + 'static> IntoIterator for SparsePartitionMap<V> {
  type Item = (NextTokenCategory, V);
  type IntoIter = Box<dyn Iterator<Item = (NextTokenCategory, V)>>;

  fn into_iter(self) -> Self::IntoIter {
    let vocab_size = self.vocab_size;
    Box::new(self.map.into_iter().map(move |(label_id, value)| {
      let label = SparsePartitionMap::<V>::static_from_id(label_id, vocab_size);
      (label, value)
    }))
  }
}

struct RuleMetadata<L> {
  /// The start index of the rules for this production label in `rules`.
  start_index: usize,
  /// One past the last index of the rules for this production label in
  /// `rules`.
  end_index: usize,
  original_label: L,
}

pub struct IndexedGrammar<T, L> {
  rules: Vec<IndexedProductionRule>,
  rule_metadata: Vec<RuleMetadata<L>>,
  vocab: AugmentedVocab<T>,
}

impl<T: Clone + Eq + Hash, L: Clone + Debug + Eq + Hash> IndexedGrammar<T, L> {
  fn build_from_grammar(
    grammar: &Grammar<T, L>,
  ) -> LRTableResult<(Self, HashMap<L, ProductionLabel>)> {
    let mut productions_iter = grammar
      .productions()
      .iter()
      .enumerate()
      .map(|(i, production)| (production, ProductionRuleIndex(i)));
    let (root_production, root_index) = productions_iter
      .next()
      .ok_or(grammar_error!(EmptyGrammar))?;

    struct LabelGroup<'a, T, L> {
      orig_label: L,
      rules: Vec<(&'a ProductionRule<T, L>, ProductionRuleIndex)>,
    }

    let root_production_label = root_production.symbol().clone();
    let mut label_map = HashMap::from_iter([(root_production_label.clone(), ProductionLabel(0))]);
    let mut label_groups = vec![LabelGroup {
      orig_label: root_production_label,
      rules: vec![(root_production, root_index)],
    }];
    let mut vocab_builder = VocabularyBuilder::new();

    for (production, original_index) in productions_iter {
      let orig_label = production.symbol().clone();
      if orig_label == *root_production.symbol() {
        return Err(grammar_error!(RootProductionRepeated));
      }
      if production
        .rule()
        .iter()
        .filter_map(|node| match node {
          ProductionNode::Production(label) => Some(label),
          ProductionNode::Terminal(..) => None,
        })
        .any(|label| label == root_production.symbol())
      {
        return Err(grammar_error!(RootProductionReferenced));
      }

      let map_len = label_map.len();
      let label = *label_map
        .entry(orig_label.clone())
        .or_insert(ProductionLabel(map_len));

      if label_map.len() != map_len {
        debug_assert_eq!(label.0, label_groups.len());
        label_groups.push(LabelGroup {
          orig_label,
          rules: vec![(production, original_index)],
        });
      } else {
        label_groups[label.0]
          .rules
          .push((production, original_index));
      }

      debug_assert_eq!(label_groups.len(), label_groups.len());
    }

    let rules = label_groups
      .iter()
      .enumerate()
      .flat_map(|(index, group)| {
        let label = ProductionLabel(index);
        let label_map = &label_map;
        let vocab_builder = &mut vocab_builder;
        group
          .rules
          .iter()
          .map(move |(production, original_index)| {
            IndexedProductionRule::new(
              label,
              production
                .rule()
                .iter()
                .map(|node| match node {
                  ProductionNode::Production(user_label) => {
                    ProductionNode::Production(*label_map.get(user_label).unwrap())
                  }
                  ProductionNode::Terminal(terminal) => {
                    ProductionNode::Terminal(vocab_builder.get_id_or_insert(terminal.clone()))
                  }
                })
                .collect(),
              *original_index,
            )
          })
          .collect_vec()
      })
      .collect_vec();

    let rule_metadata = label_groups
      .into_iter()
      .scan(0, |total, group| {
        let start_index = *total;
        let end_index = *total + group.rules.len();
        *total = end_index;
        Some(RuleMetadata {
          start_index,
          end_index,
          original_label: group.orig_label,
        })
      })
      .collect_vec();

    let vocab = vocab_builder.build();

    let indexed_grammar = Self {
      rules,
      rule_metadata,
      vocab,
    };

    indexed_grammar.verify_connected(&label_map)?;

    Ok((indexed_grammar, label_map))
  }

  #[cfg(test)]
  pub fn build_with_label_map(
    grammar: &Grammar<T, L>,
  ) -> LRTableResult<(Self, HashMap<L, ProductionLabel>)> {
    Self::build_from_grammar(grammar)
  }

  pub fn build(grammar: &Grammar<T, L>) -> LRTableResult<Self> {
    Ok(Self::build_from_grammar(grammar)?.0)
  }
}

impl<T: Clone + Eq + Hash, L: Debug> IndexedGrammar<T, L> {
  fn verify_connected(&self, label_map: &HashMap<L, ProductionLabel>) -> LRTableResult {
    let mut rule_set = self.new_production_label_set();
    let mut labels_to_explore = vec![ProductionLabel(0)];
    rule_set.set(&ProductionLabel(0));

    while let Some(label) = labels_to_explore.pop() {
      debug_assert!(rule_set.has(&label));
      for rule in self.production_rules_for_label(label) {
        for node in rule.rule() {
          if let ProductionNode::Production(label) = node
            && !rule_set.has(label)
          {
            rule_set.set(label);
            labels_to_explore.push(*label);
          }
        }
      }
    }

    if rule_set.full() {
      Ok(())
    } else {
      let mut disconnected_rules = label_map
        .iter()
        .filter(|(_, production_label)| !rule_set.has(*production_label))
        .map(|(label, _)| format!("{label:?}"));
      Err(grammar_error!(
        NotConnected,
        format!("Rules disconnected: {}", disconnected_rules.join(", "))
      ))
    }
  }
}

impl<T, L> IndexedGrammar<T, L> {
  pub fn vocab(&self) -> &AugmentedVocab<T> {
    &self.vocab
  }

  pub fn root_production_label(&self) -> ProductionLabel {
    ProductionLabel(0)
  }

  pub fn root_production_rule(&self) -> ProductionRuleId {
    ProductionRuleId(0)
  }

  pub fn all_production_labels(&self) -> impl Iterator<Item = ProductionLabel> {
    (0..self.labels_count()).map(ProductionLabel)
  }

  pub fn labels_count(&self) -> usize {
    self.rule_metadata.len()
  }

  pub fn new_sparse_augmented_vocab_map<U>(&self) -> SparseFixedSizeMap<AugmentedTokenId, U> {
    SparseFixedSizeMap::new(self.vocab.size())
  }

  fn new_production_label_set(&self) -> FixedSizeSet<ProductionLabel> {
    FixedSizeSet::new(self.labels_count())
  }

  pub fn new_production_label_map<U, F>(&self, constructor: F) -> FixedSizeMap<ProductionLabel, U>
  where
    F: FnMut() -> U,
  {
    FixedSizeMap::new_with_constructor(self.labels_count(), constructor)
  }

  pub fn new_sparse_production_label_map<U>(&self) -> SparseFixedSizeMap<ProductionLabel, U> {
    SparseFixedSizeMap::new(self.labels_count())
  }

  pub fn new_sparse_partition_closure_map<U>(&self) -> SparsePartitionMap<U> {
    SparsePartitionMap::new(self.labels_count(), self.vocab.size())
  }

  pub fn production_rule(&self, id: ProductionRuleId) -> &IndexedProductionRule {
    &self.rules[id.0]
  }

  /// Returns a range over the production rule IDs for a particular production
  /// label.
  pub fn production_rule_ids_for_label(
    &self,
    label: ProductionLabel,
  ) -> impl Iterator<Item = ProductionRuleId> {
    let meta = &self.rule_metadata[label.0];
    (meta.start_index..meta.end_index).map(ProductionRuleId)
  }

  /// Returns a range over the production rules for a particular production
  /// label.
  pub fn production_rules_for_label(
    &self,
    label: ProductionLabel,
  ) -> impl Iterator<Item = &IndexedProductionRule> {
    self
      .production_rule_ids_for_label(label)
      .map(|id| self.production_rule(id))
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    error::{BuildGrammarError, LRTableError},
    grammar::{Grammar, ProductionNode, ProductionRuleIndex},
    indexed_grammar::{IndexedGrammar, IndexedProductionRule, ProductionLabel},
    vocabulary::AugmentedVocabToken,
  };

  fn production_rules<T, L>(
    grammar: &IndexedGrammar<T, L>,
    label: ProductionLabel,
  ) -> Vec<&IndexedProductionRule> {
    grammar.production_rules_for_label(label).collect()
  }

  #[gtest]
  fn test_one_rule() {
    let grammar = Grammar::from_grammar_str("A -> a").unwrap();

    let (indexed_grammar, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    assert_eq!(indexed_grammar.labels_count(), 1);
    let label_a = *label_map.get("A").unwrap();
    let a_id = indexed_grammar.vocab().token_to_id(&b'a').unwrap();
    expect_that!(
      production_rules(&indexed_grammar, label_a),
      elements_are![&&IndexedProductionRule::new(
        label_a,
        vec![ProductionNode::Terminal(AugmentedVocabToken::Token(a_id))],
        ProductionRuleIndex(0)
      )]
    );
  }

  #[gtest]
  fn test_root_production_duplicated() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a
         A -> b"#,
    )
    .unwrap();

    let grammar = IndexedGrammar::build(&grammar);
    expect_that!(
      grammar.err(),
      some(pat!(LRTableError::BuildGrammar(pat!(
        BuildGrammarError::RootProductionRepeated
      ))))
    );
  }

  #[gtest]
  fn test_root_production_referenced() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> C
         C -> A"#,
    )
    .unwrap();

    let grammar = IndexedGrammar::build(&grammar);
    expect_that!(
      grammar.err(),
      some(pat!(LRTableError::BuildGrammar(pat!(
        BuildGrammarError::RootProductionReferenced
      ))))
    );
  }

  #[gtest]
  fn test_grammar_not_connected() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> C
         C -> c
         D -> E
         D -> !
         E -> c D"#,
    )
    .unwrap();

    let grammar = IndexedGrammar::build(&grammar);
    expect_that!(
      grammar.err(),
      some(pat!(LRTableError::BuildGrammar(pat!(
        BuildGrammarError::NotConnected(all![contains_regex(r"\bD\b"), contains_regex(r"\bE\b")])
      ))))
    );
  }

  #[gtest]
  fn test_two_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a
         A -> b"#,
    )
    .unwrap();

    let (indexed_grammar, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    assert_eq!(indexed_grammar.labels_count(), 2);
    let label_a = *label_map.get("A").unwrap();
    let a_id = indexed_grammar.vocab().token_to_id(&b'a').unwrap();
    let b_id = indexed_grammar.vocab().token_to_id(&b'b').unwrap();
    expect_that!(
      production_rules(&indexed_grammar, label_a),
      elements_are![
        &&IndexedProductionRule::new(
          label_a,
          vec![ProductionNode::Terminal(AugmentedVocabToken::Token(a_id))],
          ProductionRuleIndex(1)
        ),
        &&IndexedProductionRule::new(
          label_a,
          vec![ProductionNode::Terminal(AugmentedVocabToken::Token(b_id))],
          ProductionRuleIndex(2)
        )
      ]
    );
  }

  #[gtest]
  fn test_vocab_set_single_token() {
    let grammar = Grammar::from_grammar_str("A -> a").unwrap();
    let indexed_grammar = IndexedGrammar::build(&grammar).unwrap();

    let vocab = indexed_grammar.vocab();
    expect_eq!(vocab.size(), 3);
  }
}
