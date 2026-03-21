use std::{
  fmt::{Debug, Display},
  hash::Hash,
};

use crate::{
  first_map::FirstTable,
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, IndexedProductionNode, ProductionLabel, ProductionRuleId},
  vocabulary::{AugmentedVocab, AugmentedVocabToken, VocabSet},
};

/// Returns the production label of the first node of `rule` if it is a
/// production node, otherwise `None`.
pub fn maybe_first_production_label<T>(
  rule: &[ProductionNode<T, ProductionLabel>],
) -> Option<ProductionLabel> {
  if rule.is_empty() {
    return None;
  }

  match rule[0] {
    ProductionNode::Production(label) => Some(label),
    _ => None,
  }
}

/// Given a list of nodes, and the set of tokens which may succeed this list of
/// nodes, returns the set of possible first tokens which may immediately
/// succeed the production rule at `rule[0]`.
///
/// The first node of `rule` must be a production rule.
pub fn follow_set_for_rule<T>(
  rule: &[IndexedProductionNode],
  rule_follow_set: &VocabSet,
  first_map: &FirstTable,
  vocab: &AugmentedVocab<T>,
) -> VocabSet {
  let mut token_set = VocabSet::new(vocab);
  for node in rule {
    match node {
      ProductionNode::Production(label) => {
        let first_set = first_map.first_set(*label);
        token_set.merge(first_set);
        // If this set does not contain epsilon, then we can stop
        if !first_set.has(AugmentedVocabToken::Epsilon) {
          return token_set;
        }
        // If `first_set` had epsilon, we need to remove it from `token_set`.
        token_set.clear(AugmentedVocabToken::Epsilon);
      }
      ProductionNode::Terminal(AugmentedVocabToken::Epsilon) => {}
      ProductionNode::Terminal(token) => {
        token_set.set(*token);
        return token_set;
      }
    }
  }

  // If we made it through the whole `rule`, then it's possible that the
  // immediate next token bypasses the rest of the rules. Merge with our own
  // next tokens.
  token_set.merge(rule_follow_set);
  token_set
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
  production_id: ProductionRuleId,
  position: usize,
  follow_set: VocabSet,
}

impl Position {
  pub fn rule(&self) -> ProductionRuleId {
    self.production_id
  }

  pub fn follow_set(&self) -> &VocabSet {
    &self.follow_set
  }

  /// Returns a tuple of (production rule, index), where the index is the
  /// offset of the current position from the start of the production rule.
  pub fn position(&self) -> (ProductionRuleId, usize) {
    (self.production_id, self.position)
  }

  pub fn at_end_of_rule<T>(&self, grammar: &IndexedGrammar<T>) -> bool {
    self.next_node(grammar).is_none()
  }

  /// Returns the next node at this position, or `None` if this position is at
  /// the end of its rule.
  pub fn next_node<'a, T>(
    &self,
    grammar: &'a IndexedGrammar<T>,
  ) -> Option<&'a IndexedProductionNode> {
    let production = grammar.production_rule(self.production_id);
    production.rule()[self.position..]
      .iter()
      .find(|node| !matches!(node, ProductionNode::Terminal(AugmentedVocabToken::Epsilon)))
  }

  /// Advances this position to the next node. This must not be called on
  /// positions that are already at the end of their rule.
  pub fn advance<T>(&mut self, grammar: &IndexedGrammar<T>) {
    debug_assert!(!self.at_end_of_rule(grammar));
    self.position += 1;
  }

  pub fn advance_all<'a, T, I>(iter: I, grammar: &IndexedGrammar<T>)
  where
    T: 'a,
    I: Iterator<Item = &'a mut Self>,
  {
    for position in iter {
      position.advance(grammar);
    }
  }

  pub fn new_from_start_with_follow_set(
    production_id: ProductionRuleId,
    follow_set: VocabSet,
  ) -> Self {
    Self {
      production_id,
      position: 0,
      follow_set,
    }
  }

  #[cfg(test)]
  pub fn new_at_pos(
    production_id: ProductionRuleId,
    position: usize,
    follow_set: VocabSet,
  ) -> Self {
    Self {
      production_id,
      position,
      follow_set,
    }
  }

  pub fn new_top_level<T>(production_id: ProductionRuleId, vocab: &AugmentedVocab<T>) -> Self {
    let mut follow_set = VocabSet::new(vocab);
    follow_set.set(AugmentedVocabToken::EndOfStream);
    Self::new_from_start_with_follow_set(production_id, follow_set)
  }

  /// Returns the production label of the next node of this rule, if that node
  /// is a production label. If the next node is a terminal, or `position` is
  /// already at the end of this rule, returns `None`.
  pub fn next_production_label<T>(
    &self,
    grammar: &IndexedGrammar<T>,
    first_map: &FirstTable,
  ) -> Option<(ProductionLabel, VocabSet)> {
    let production = grammar.production_rule(self.production_id);

    let label = maybe_first_production_label(&production.rule()[self.position..])?;

    let follow_set = follow_set_for_rule(
      &production.rule()[self.position + 1..],
      &self.follow_set,
      first_map,
      grammar.vocab(),
    );
    Some((label, follow_set))
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?} at {} [{}]",
      self.production_id, self.position, self.follow_set
    )
  }
}

impl Debug for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?} at {} [{:?}]",
      self.production_id, self.position, self.follow_set
    )
  }
}
