use itertools::Itertools;

use crate::{
  first_map::FirstTable,
  fixed_map::SparseFixedSizeMap,
  indexed_grammar::{IndexedGrammar, ProductionLabel, SparsePartitionMap},
  kernel::Kernel,
  position::{Position, follow_set_for_rule, maybe_first_production_label},
  vocabulary::VocabSet,
};

/// Given a kernel, which is a list of `Position`, returns a map over all
/// production labels which are transitively connected to the next nodes of
/// any rule in the kernel, and the follow sets for those such production
/// labels.
fn closure_follow_sets<T>(
  kernel: &Kernel,
  grammar: &IndexedGrammar<T>,
  first_map: &FirstTable,
) -> SparseFixedSizeMap<ProductionLabel, VocabSet> {
  // A map from `ProductionLabel` to follow set. We only need to track the next
  // token set for each production label, not each individual rule.
  let mut production_follow_sets = grammar.new_sparse_production_label_map::<VocabSet>();

  // It may seem we are at risk of duplicating rules in `kernel` if they are in
  // position 0 and are also circularly referenced. However, the only rule at
  // position 0 in the kernel is the root position, which we require is not
  // referenced by any other rule.

  {
    let mut productions_to_explore = Vec::new();

    for production_rule in kernel.positions() {
      if let Some((label, follow_set)) = production_rule.next_production_label(grammar, first_map) {
        production_follow_sets
          .get_mut_or_insert_with(&label, || {
            productions_to_explore.push(label);
            VocabSet::new(grammar.vocab())
          })
          .merge(&follow_set);
      }
    }

    // Recursively explore the whole closure tree until all labels have been found.
    while let Some(label) = productions_to_explore.pop() {
      for production_rule in grammar.production_rules_for_label(label) {
        if let Some(sub_label) = maybe_first_production_label(production_rule.rule()) {
          production_follow_sets.get_mut_or_insert_with(&sub_label, || {
            productions_to_explore.push(sub_label);
            VocabSet::new(grammar.vocab())
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
      for production_rule in grammar.production_rules_for_label(label) {
        let Some(sub_label) = maybe_first_production_label(production_rule.rule()) else {
          continue;
        };

        let follow_set = production_follow_sets.get(&label).unwrap();
        let sub_follow_set = follow_set_for_rule(
          &production_rule.rule()[1..],
          follow_set,
          first_map,
          grammar.vocab(),
        );

        changed = production_follow_sets
          .get_mut(&sub_label)
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

/// Given a kernel, computes a partition over the positions of the kernel's
/// closure grouped by next nodes (either productions or terminals). All
/// positions at the end of their rules are grouped together under `None`.
pub fn partition_closure_by_next_node<T>(
  kernel: &Kernel,
  grammar: &IndexedGrammar<T>,
  first_map: &FirstTable,
) -> SparsePartitionMap<Vec<Position>> {
  kernel
    .positions()
    .cloned()
    .chain(
      closure_follow_sets(kernel, grammar, first_map)
        .into_iter()
        .flat_map(|(label, follow_set)| {
          grammar
            .production_rule_ids_for_label(label)
            .map(move |production_id| {
              Position::new_from_start_with_follow_set(production_id, follow_set.clone())
            })
        }),
    )
    .fold(
      grammar.new_sparse_partition_closure_map(),
      |mut map, position| {
        map
          .get_mut_or_default(&position.next_node(grammar).cloned())
          .push(position.clone());
        map
      },
    )
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::{
    augmented_vocab_token::AugmentedVocabToken,
    first_map::FirstTable,
    grammar::{Grammar, ProductionNode},
    indexed_grammar::{IndexedGrammar, NextTokenCategory, ProductionLabel},
    kernel::Kernel,
    position::Position,
    vocabulary::VocabSet,
  };

  fn closure_follow_sets<T>(
    position: Position,
    grammar: &IndexedGrammar<T>,
  ) -> Vec<(ProductionLabel, VocabSet)> {
    let first_map = FirstTable::build_from_grammar(grammar);
    let kernel = Kernel::new(vec![position]);
    let map = crate::closure::closure_follow_sets(&kernel, grammar, &first_map);
    map
      .iter()
      .map(|(label, follow_set)| (label, follow_set.clone()))
      .collect()
  }

  fn partition_closure_by_next_node<T>(
    kernel: impl IntoIterator<Item = Position>,
    grammar: &IndexedGrammar<T>,
  ) -> Vec<(NextTokenCategory, Vec<Position>)> {
    let first_map = FirstTable::build_from_grammar(grammar);
    let kernel = Kernel::new(kernel.into_iter().collect());
    crate::closure::partition_closure_by_next_node(&kernel, grammar, &first_map)
      .into_iter()
      .collect()
  }

  #[gtest]
  fn test_no_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> b"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_at_pos(
          production_id_a,
          0,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &indexed
      ),
      elements_are![&(
        label_b,
        VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
      )]
    );
  }

  #[gtest]
  fn test_nonzero_pos_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> a B
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &indexed
      ),
      elements_are![&(
        label_b,
        VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
      )]
    );
  }

  #[gtest]
  fn test_circular_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a B
         B -> A"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &indexed
      ),
      unordered_elements_are![
        &(
          label_a,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &(
          label_b,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    let label_c = *label_map.get("C").unwrap();
    let label_d = *label_map.get("D").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_at_pos(
          production_id_a,
          1,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &indexed
      ),
      unordered_elements_are![
        &(
          label_b,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &(
          label_c,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
        &(
          label_d,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        ),
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
      elements_are![&(
        label_b,
        VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
      )]
    );
  }

  #[gtest]
  fn test_closure_skip_all_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B C D
         B -> b
         B -> !
         C -> c
         C -> !
         D -> d
         D -> !"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let a_rules = indexed.production_rule_ids_for_label(label_a).collect_vec();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(a_rules[0], indexed.vocab()),
        &indexed
      ),
      elements_are![&(
        label_b,
        VocabSet::from_iter(
          [b'c'.into(), b'd'.into(), AugmentedVocabToken::EndOfStream],
          indexed.vocab()
        )
      )]
    );
  }

  #[gtest]
  fn test_closure_follow_set_before_terminal() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B c
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
      elements_are![&(label_b, VocabSet::from_iter([b'c'.into()], indexed.vocab()))]
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
      elements_are![&(label_b, VocabSet::from_iter([b'c'.into()], indexed.vocab()))]
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
      elements_are![&(
        label_b,
        VocabSet::from_iter([b'c'.into(), b'd'.into()], indexed.vocab())
      )]
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_b = *label_map.get("B").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
      elements_are![&(
        label_b,
        VocabSet::from_iter(
          [b'c'.into(), b'd'.into(), AugmentedVocabToken::EndOfStream],
          indexed.vocab()
        )
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

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed
      .production_rule_ids_for_label(label_a)
      .next()
      .unwrap();
    let label_s = *label_map.get("S").unwrap();
    let label_p = *label_map.get("P").unwrap();
    let label_v = *label_map.get("V").unwrap();
    expect_that!(
      closure_follow_sets(
        Position::new_top_level(production_id_a, indexed.vocab()),
        &indexed
      ),
      unordered_elements_are![
        &(
          label_s,
          VocabSet::from_iter(
            [b'p'.into(), AugmentedVocabToken::EndOfStream],
            indexed.vocab()
          )
        ),
        &(
          label_p,
          VocabSet::from_iter(
            [b'p'.into(), b'x'.into(), AugmentedVocabToken::EndOfStream],
            indexed.vocab()
          )
        ),
        &(
          label_v,
          VocabSet::from_iter(
            [b'p'.into(), b'x'.into(), AugmentedVocabToken::EndOfStream],
            indexed.vocab()
          )
        ),
      ]
    );
  }

  #[gtest]
  fn test_closure_partition_simple() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a C
         C -> b
         C -> c"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_b = *label_map.get("B").unwrap();
    let b_rules = indexed.production_rule_ids_for_label(label_b).collect_vec();
    let label_c = *label_map.get("C").unwrap();
    let c_rules = indexed.production_rule_ids_for_label(label_c).collect_vec();

    let b_id = indexed.vocab().token_to_id(&b'b');
    let c_id = indexed.vocab().token_to_id(&b'c');
    expect_that!(
      partition_closure_by_next_node(
        [Position::new_at_pos(
          b_rules[0],
          1,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        )],
        &indexed
      ),
      unordered_elements_are![
        (
          some(eq(&ProductionNode::Production(label_c))),
          elements_are![property!(&Position.position(), (b_rules[0], 1))]
        ),
        (
          some(eq(&ProductionNode::Terminal(b_id.into()))),
          elements_are![property!(&Position.position(), (c_rules[0], 0))]
        ),
        (
          some(eq(&ProductionNode::Terminal(c_id.into()))),
          elements_are![property!(&Position.position(), (c_rules[1], 0))]
        )
      ]
    );
  }

  #[gtest]
  fn test_closure_partition_end_of_rule() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_b = *label_map.get("B").unwrap();
    let b_rules = indexed.production_rule_ids_for_label(label_b).collect_vec();
    expect_that!(
      partition_closure_by_next_node(
        [Position::new_at_pos(
          b_rules[0],
          1,
          VocabSet::from_iter([AugmentedVocabToken::EndOfStream], indexed.vocab())
        )],
        &indexed
      ),
      elements_are![(
        none(),
        elements_are![property!(&Position.position(), (b_rules[0], 1))]
      )]
    );
  }

  #[gtest]
  fn test_closure_partition_skip_all_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B C D
         B -> b
         B -> !
         C -> c
         C -> !
         D -> d
         D -> !"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_a = *label_map.get("A").unwrap();
    let a_rules = indexed.production_rule_ids_for_label(label_a).collect_vec();
    let label_b = *label_map.get("B").unwrap();
    let b_rules = indexed.production_rule_ids_for_label(label_b).collect_vec();

    let b_id = indexed.vocab().token_to_id(&b'b');
    expect_that!(
      partition_closure_by_next_node(
        [Position::new_top_level(a_rules[0], indexed.vocab())],
        &indexed
      ),
      unordered_elements_are![
        (
          none(),
          elements_are![property!(&Position.position(), (b_rules[1], 0))]
        ),
        (
          some(eq(&ProductionNode::Production(label_b))),
          elements_are![property!(&Position.position(), (a_rules[0], 0))]
        ),
        (
          some(eq(&ProductionNode::Terminal(b_id.into()))),
          elements_are![property!(&Position.position(), (b_rules[0], 0))]
        )
      ]
    );
  }
}
