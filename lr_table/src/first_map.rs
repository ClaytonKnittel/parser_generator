use crate::{
  fixed_map::FixedSizeMap,
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionLabel},
  vocab_set::VocabSet,
  vocabulary::{AugmentedVocab, Vocabulary},
};

/// A map from production labels to the set of possible first tokens.
pub struct FirstTable<T> {
  map: FixedSizeMap<ProductionLabel, VocabSet<AugmentedVocab<T>>>,
}

impl<T: Vocabulary> FirstTable<T> {
  /// Does one round of updates to the next token map. Returns `true` if any
  /// changes were made, or `false` if none were.
  fn propagate_map(
    map: &mut FixedSizeMap<ProductionLabel, VocabSet<AugmentedVocab<T>>>,
    grammar: &IndexedGrammar<T>,
  ) -> bool {
    let mut changed = false;

    for label in grammar.all_production_labels() {
      'productions_loop: for rule in grammar.production_rules_for_label(label) {
        for node in rule.rule() {
          match node {
            ProductionNode::Production(node_label) => {
              let mut node_first_set = map.get(*node_label).clone();
              let has_epsilon = node_first_set.get(&AugmentedVocab::Epsilon);
              node_first_set.clear(&AugmentedVocab::Epsilon);
              changed = map.get_mut(label).merge(&node_first_set) || changed;
              if !has_epsilon {
                continue 'productions_loop;
              }
            }
            ProductionNode::Terminal(AugmentedVocab::Epsilon) => {}
            ProductionNode::Terminal(terminal) => {
              let first_set = map.get_mut(label);
              if !first_set.get(terminal) {
                map.get_mut(label).set(terminal);
                changed = true;
              }
              continue 'productions_loop;
            }
          }
        }

        // If we passed all nodes of this rule, then it's possible this rule
        // evaluates to epsilon.
        map.get_mut(label).set(&AugmentedVocab::Epsilon);
      }
    }

    changed
  }

  pub fn build_from_grammar(grammar: &IndexedGrammar<T>) -> Self {
    let mut map = grammar.new_production_label_map();
    while Self::propagate_map(&mut map, grammar) {}

    Self { map }
  }

  pub fn first_set(&self, label: ProductionLabel) -> &VocabSet<AugmentedVocab<T>> {
    self.map.get(label)
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    first_map::FirstTable, grammar::Grammar, indexed_grammar::IndexedGrammar, vocab_set::VocabSet,
    vocabulary::AugmentedVocab,
  };

  #[gtest]
  fn test_one_rule_grammar() {
    let grammar = Grammar::from_grammar_str(r#"A -> a"#).unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();

    let first_table = FirstTable::build_from_grammar(&indexed);
    expect_eq!(
      first_table.first_set(label_a),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a')])
    );
  }

  #[gtest]
  fn test_sequence_of_rules_grammar() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> C
         C -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let label_b = *label_map.get("B").unwrap();
    let label_c = *label_map.get("C").unwrap();

    let first_table = FirstTable::build_from_grammar(&indexed);
    expect_eq!(
      first_table.first_set(label_a),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a')])
    );
    expect_eq!(
      first_table.first_set(label_b),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a')])
    );
    expect_eq!(
      first_table.first_set(label_c),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a')])
    );
  }

  #[gtest]
  fn test_tree_of_rules_grammar() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> C
         B -> D
         C -> a
         D -> b"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let label_b = *label_map.get("B").unwrap();
    let label_c = *label_map.get("C").unwrap();
    let label_d = *label_map.get("D").unwrap();

    let first_table = FirstTable::build_from_grammar(&indexed);
    expect_eq!(
      first_table.first_set(label_a),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a'), AugmentedVocab::Token(b'b')])
    );
    expect_eq!(
      first_table.first_set(label_b),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a'), AugmentedVocab::Token(b'b')])
    );
    expect_eq!(
      first_table.first_set(label_c),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a')])
    );
    expect_eq!(
      first_table.first_set(label_d),
      &VocabSet::from_iter([AugmentedVocab::Token(b'b')])
    );
  }

  #[gtest]
  fn test_skip_one_epsilon() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B a
         B -> !
         B -> b"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let label_b = *label_map.get("B").unwrap();

    let first_table = FirstTable::build_from_grammar(&indexed);
    expect_eq!(
      first_table.first_set(label_a),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a'), AugmentedVocab::Token(b'b')])
    );
    expect_eq!(
      first_table.first_set(label_b),
      &VocabSet::from_iter([AugmentedVocab::Token(b'b'), AugmentedVocab::Epsilon])
    );
  }

  #[gtest]
  fn test_epsilon_propagate() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> !
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let label_b = *label_map.get("B").unwrap();

    let first_table = FirstTable::build_from_grammar(&indexed);
    expect_eq!(
      first_table.first_set(label_a),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a'), AugmentedVocab::Epsilon])
    );
    expect_eq!(
      first_table.first_set(label_b),
      &VocabSet::from_iter([AugmentedVocab::Token(b'a'), AugmentedVocab::Epsilon])
    );
  }

  #[gtest]
  fn test_recursive_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a
         A -> B b
         B -> c
         B -> !
         B -> A"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let label_b = *label_map.get("B").unwrap();

    let first_table = FirstTable::build_from_grammar(&indexed);
    expect_eq!(
      first_table.first_set(label_a),
      &VocabSet::from_iter([
        AugmentedVocab::Token(b'a'),
        AugmentedVocab::Token(b'b'),
        AugmentedVocab::Token(b'c')
      ])
    );
    expect_eq!(
      first_table.first_set(label_b),
      &VocabSet::from_iter([
        AugmentedVocab::Token(b'a'),
        AugmentedVocab::Token(b'b'),
        AugmentedVocab::Token(b'c'),
        AugmentedVocab::Epsilon
      ])
    );
  }
}
