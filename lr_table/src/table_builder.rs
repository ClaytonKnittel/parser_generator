use std::fmt::{Debug, Display};

use itertools::Itertools;

use crate::{
  first_map::FirstTable,
  fixed_map::SparseFixedSizeMap,
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionLabel, ProductionRuleId},
  vocab_set::VocabSet,
  vocabulary::{AugmentedVocab, Vocabulary},
};

/// Returns the production label of the first node of `rule` if it is a
/// production node, otherwise `None`.
fn maybe_first_production_label<T>(
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
fn follow_set_for_rule<T: Vocabulary>(
  rule: &[ProductionNode<T, ProductionLabel>],
  rule_follow_set: &VocabSet<AugmentedVocab<T>>,
  first_map: &FirstTable<T>,
) -> VocabSet<AugmentedVocab<T>> {
  let mut token_set = VocabSet::new();
  for node in rule {
    match node {
      ProductionNode::Production(label) => {
        let first_set = first_map.first_set(*label);
        token_set.merge(first_set);
        // If this set does not contain epsilon, then we can stop
        if !first_set.get(&AugmentedVocab::Epsilon) {
          return token_set;
        }
        // If `first_set` had epsilon, we need to remove it from `token_set`.
        token_set.clear(&AugmentedVocab::Epsilon);
      }
      ProductionNode::Terminal(AugmentedVocab::Epsilon) => {}
      ProductionNode::Terminal(token) => {
        token_set.set(token);
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

#[derive(PartialEq, Eq)]
struct ProductionRulePos<T> {
  production_id: ProductionRuleId,
  position: usize,
  follow_set: VocabSet<AugmentedVocab<T>>,
}

impl<T: Vocabulary> ProductionRulePos<T> {
  fn new_from_start_with_follow_set(
    production_id: ProductionRuleId,
    follow_set: VocabSet<AugmentedVocab<T>>,
  ) -> Self {
    Self {
      production_id,
      position: 0,
      follow_set,
    }
  }

  #[cfg(test)]
  fn new_at_pos(
    production_id: ProductionRuleId,
    position: usize,
    follow_set: VocabSet<AugmentedVocab<T>>,
  ) -> Self {
    Self {
      production_id,
      position,
      follow_set,
    }
  }

  fn new_top_level(production_id: ProductionRuleId) -> Self {
    let mut follow_set = VocabSet::new();
    follow_set.set(&AugmentedVocab::<T>::EndOfStream);
    Self::new_from_start_with_follow_set(production_id, follow_set)
  }

  /// Returns the production label of the next node of this rule, if that node
  /// is a production label. If the next node is a terminal, or `position` is
  /// already at the end of this rule, returns `None`.
  fn next_production_label(
    &self,
    grammar: &IndexedGrammar<T>,
    first_map: &FirstTable<T>,
  ) -> Option<(ProductionLabel, VocabSet<AugmentedVocab<T>>)> {
    let production = grammar.production_rule(self.production_id);

    let label = maybe_first_production_label(&production.rule()[self.position..])?;

    let follow_set = follow_set_for_rule(
      &production.rule()[self.position + 1..],
      &self.follow_set,
      first_map,
    );
    Some((label, follow_set))
  }
}

impl<T> Clone for ProductionRulePos<T> {
  fn clone(&self) -> Self {
    Self {
      production_id: self.production_id,
      position: self.position,
      follow_set: self.follow_set.clone(),
    }
  }
}

impl<T: Vocabulary + Display> Display for ProductionRulePos<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?} at {} [{}]",
      self.production_id, self.position, self.follow_set
    )
  }
}

impl<T: Vocabulary + Display> Debug for ProductionRulePos<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

/// Given a kernel, which is a list of `ProductionRulePos`, returns a map over
/// all production labels which are transitively connected to the next nodes of
/// any rule in the kernel, and the follow sets for those such production
/// labels.
fn closure_follow_sets<T: Vocabulary>(
  kernel: impl IntoIterator<Item = ProductionRulePos<T>>,
  grammar: &IndexedGrammar<T>,
  first_map: &FirstTable<T>,
) -> SparseFixedSizeMap<ProductionLabel, VocabSet<AugmentedVocab<T>>> {
  // A map from `ProductionLabel` to follow set. We only need to track the next
  // token set for each production label, not each individual rule.
  let mut production_follow_sets =
    grammar.new_sparse_production_label_map::<VocabSet<AugmentedVocab<T>>>();

  // It may seem we are at risk of duplicating rules in `kernel` if they are in
  // position 0 and are also circularly referenced. However, the only rule at
  // position 0 in the kernel is the root position, which we require is not
  // referenced by any other rule.

  {
    let mut productions_to_explore = Vec::new();

    for production_rule in kernel {
      if let Some((label, follow_set)) = production_rule.next_production_label(grammar, first_map) {
        production_follow_sets
          .get_mut_or_insert_with(label, || {
            productions_to_explore.push(label);
            VocabSet::new()
          })
          .merge(&follow_set);
      }
    }

    // Recursively explore the whole closure tree until all labels have been found.
    while let Some(label) = productions_to_explore.pop() {
      for production_rule_id in grammar.productions_for_label(label) {
        let production_rule = grammar.production_rule(production_rule_id);

        if let Some(sub_label) = maybe_first_production_label(production_rule.rule()) {
          production_follow_sets.get_mut_or_insert_with(sub_label, || {
            productions_to_explore.push(sub_label);
            VocabSet::new()
          });
        }
      }
    }
  }

  let production_labels = production_follow_sets
    .iter()
    .map(|(label, _)| label)
    .collect_vec();

  loop {
    let mut changed = false;

    for &label in &production_labels {
      for production_rule_id in grammar.productions_for_label(label) {
        let production_rule = grammar.production_rule(production_rule_id);

        let Some(sub_label) = maybe_first_production_label(production_rule.rule()) else {
          continue;
        };

        let follow_set = production_follow_sets.get(label).unwrap();
        let sub_follow_set =
          follow_set_for_rule(&production_rule.rule()[1..], follow_set, first_map);

        changed = production_follow_sets
          .get_mut(sub_label)
          .unwrap()
          .merge(&sub_follow_set)
          || changed;
      }
    }

    if !changed {
      break;
    }
  }

  production_follow_sets
}

fn generate_actions<T: Vocabulary + Display>(indexed_grammar: &IndexedGrammar<T>) {
  let first_set = FirstTable::build_from_grammar(indexed_grammar);

  let root_label = indexed_grammar.root_production_label();
  for rule_id in indexed_grammar.productions_for_label(root_label) {
    for (label, follow_set) in closure_follow_sets(
      [ProductionRulePos::new_top_level(rule_id)],
      indexed_grammar,
      &first_set,
    )
    .iter()
    {
      println!("Position: {label:?}: {follow_set}");
    }
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::{
    first_map::FirstTable,
    grammar::Grammar,
    indexed_grammar::{IndexedGrammar, ProductionLabel},
    table_builder::ProductionRulePos,
    vocab_set::VocabSet,
    vocabulary::{AugmentedVocab, Vocabulary},
  };

  fn closure_follow_sets<T: Vocabulary>(
    position: ProductionRulePos<T>,
    grammar: &IndexedGrammar<T>,
  ) -> Vec<(ProductionLabel, VocabSet<AugmentedVocab<T>>)> {
    let first_map = FirstTable::build_from_grammar(grammar);
    let map = crate::table_builder::closure_follow_sets([position], grammar, &first_map);
    map
      .iter()
      .map(|(label, follow_set)| (label, follow_set.clone()))
      .collect()
  }

  #[gtest]
  fn test_no_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> b
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      is_empty()
    );
  }

  #[gtest]
  fn test_closure_at_end() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    expect_that!(
      closure_follow_sets(
        ProductionRulePos::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocab::EndOfStream])
        ),
        &indexed
      ),
      is_empty()
    );
  }

  #[gtest]
  fn test_closure_at_start() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        ProductionRulePos::new_at_pos(
          production_id_a,
          0,
          VocabSet::from_iter([AugmentedVocab::EndOfStream])
        ),
        &indexed
      ),
      elements_are![&(label_b, VocabSet::from_iter([AugmentedVocab::EndOfStream]))]
    );
  }

  #[gtest]
  fn test_nonzero_pos_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a B
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        ProductionRulePos::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocab::EndOfStream])
        ),
        &indexed
      ),
      elements_are![&(label_b, VocabSet::from_iter([AugmentedVocab::EndOfStream]))]
    );
  }

  #[gtest]
  fn test_circular_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a B
         B -> A"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        ProductionRulePos::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocab::EndOfStream])
        ),
        &indexed
      ),
      unordered_elements_are![
        &(label_a, VocabSet::from_iter([AugmentedVocab::EndOfStream])),
        &(label_b, VocabSet::from_iter([AugmentedVocab::EndOfStream])),
      ]
    );
  }

  #[gtest]
  fn test_long_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a B
         B -> C
         C -> D
         D -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    let label_c = *label_map.get("C").unwrap();
    let label_d = *label_map.get("D").unwrap();
    expect_that!(
      closure_follow_sets(
        ProductionRulePos::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocab::EndOfStream])
        ),
        &indexed
      ),
      unordered_elements_are![
        &(label_b, VocabSet::from_iter([AugmentedVocab::EndOfStream])),
        &(label_c, VocabSet::from_iter([AugmentedVocab::EndOfStream])),
        &(label_d, VocabSet::from_iter([AugmentedVocab::EndOfStream])),
      ]
    );
  }

  #[gtest]
  fn test_closure_multiple_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a
         B -> b"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      elements_are![&(label_b, VocabSet::from_iter([AugmentedVocab::EndOfStream]))]
    );
  }

  #[gtest]
  fn test_closure_follow_set_before_terminal() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B c
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      elements_are![&(label_b, VocabSet::from_iter([b'c']))]
    );
  }

  #[gtest]
  fn test_closure_follow_set_before_production_label() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B C d
         B -> b
         C -> c"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      elements_are![&(label_b, VocabSet::from_iter([b'c']))]
    );
  }

  #[gtest]
  fn test_closure_follow_set_before_production_label_with_epsilon() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B C d
         B -> b
         C -> c
         C -> !"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      elements_are![&(label_b, VocabSet::from_iter([b'c', b'd']))]
    );
  }

  #[gtest]
  fn test_closure_follow_set_epsilon_until_end() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B C D
         B -> b
         C -> c
         C -> !
         D -> d
         D -> !"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      elements_are![&(
        label_b,
        VocabSet::from_iter([b'c'.into(), b'd'.into(), AugmentedVocab::EndOfStream])
      )]
    );
  }

  #[gtest]
  fn test_closure_follow_set_sum_product_grammar() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> S
         S -> S p V
         S -> P
         P -> P x V
         P -> V
         V -> a
         V -> b
         V -> c"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_s = *label_map.get("S").unwrap();
    let label_p = *label_map.get("P").unwrap();
    let label_v = *label_map.get("V").unwrap();
    expect_that!(
      closure_follow_sets(ProductionRulePos::new_top_level(production_id_a), &indexed),
      unordered_elements_are![
        &(
          label_s,
          VocabSet::from_iter([b'p'.into(), AugmentedVocab::EndOfStream])
        ),
        &(
          label_p,
          VocabSet::from_iter([b'p'.into(), b'x'.into(), AugmentedVocab::EndOfStream])
        ),
        &(
          label_v,
          VocabSet::from_iter([b'p'.into(), b'x'.into(), AugmentedVocab::EndOfStream])
        ),
      ]
    );
  }
}
