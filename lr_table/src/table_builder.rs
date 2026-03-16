use crate::{
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionRuleId},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProductionPosition {
  production_id: ProductionRuleId,
  position: usize,
}

impl ProductionPosition {
  fn new(production_id: ProductionRuleId, position: usize) -> Self {
    Self {
      production_id,
      position,
    }
  }
}

fn closure<T>(
  position: ProductionPosition,
  grammar: &IndexedGrammar<T>,
) -> impl Iterator<Item = ProductionPosition> {
  let mut positions = Vec::new();
  let mut stack = vec![position];
  let mut label_set = grammar.production_label_set();

  if position.position == 0 {
    label_set.set(grammar.rule_label(position.production_id));
  }

  while let Some(pos) = stack.pop() {
    positions.push(pos);
    let production = grammar.production_rule(pos.production_id);
    if pos.position >= production.rule().len() {
      continue;
    }

    if let ProductionNode::Production(label) = &production.rule()[pos.position] {
      for production_id in grammar.productions_for_label(*label) {
        let rule_label = grammar.rule_label(production_id);
        if label_set.get(rule_label) {
          continue;
        }

        label_set.set(rule_label);
        stack.push(ProductionPosition {
          production_id,
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
    grammar::Grammar,
    indexed_grammar::IndexedGrammar,
    table_builder::{ProductionPosition, closure},
  };

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
      closure(ProductionPosition::new(production_id_a, 0), &indexed).collect_vec(),
      elements_are![&ProductionPosition::new(production_id_a, 0)]
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
      closure(ProductionPosition::new(production_id_a, 1), &indexed).collect_vec(),
      elements_are![&ProductionPosition::new(production_id_a, 1)]
    );
  }

  #[gtest]
  fn test_closure_at_start() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> A"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    let production_id_b = indexed.productions_for_label(label_b).next().unwrap();
    expect_that!(
      closure(ProductionPosition::new(production_id_a, 0), &indexed).collect_vec(),
      unordered_elements_are![
        &ProductionPosition::new(production_id_a, 0),
        &ProductionPosition::new(production_id_b, 0),
      ]
    );
  }

  #[gtest]
  fn test_small_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a"#,
    )
    .unwrap();

    let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    let label_a = *label_map.get("A").unwrap();
    let production_id_a = indexed.productions_for_label(label_a).next().unwrap();
    let label_b = *label_map.get("B").unwrap();
    let production_id_b = indexed.productions_for_label(label_b).next().unwrap();
    expect_that!(
      closure(ProductionPosition::new(production_id_a, 0), &indexed).collect_vec(),
      unordered_elements_are![
        &ProductionPosition::new(production_id_a, 0),
        &ProductionPosition::new(production_id_b, 0),
      ]
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
    let production_id_b = indexed.productions_for_label(label_b).next().unwrap();
    expect_that!(
      closure(ProductionPosition::new(production_id_a, 1), &indexed).collect_vec(),
      unordered_elements_are![
        &ProductionPosition::new(production_id_a, 1),
        &ProductionPosition::new(production_id_b, 0),
      ]
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
    let production_id_b = indexed.productions_for_label(label_b).next().unwrap();
    expect_that!(
      closure(
        ProductionPosition {
          production_id: production_id_a,
          position: 1
        },
        &indexed
      )
      .collect_vec(),
      unordered_elements_are![
        &ProductionPosition::new(production_id_a, 1),
        &ProductionPosition::new(production_id_a, 0),
        &ProductionPosition::new(production_id_b, 0),
      ]
    );
  }

  #[gtest]
  fn test_large_closure() {
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
    let production_id_b = indexed.productions_for_label(label_b).next().unwrap();
    let label_c = *label_map.get("C").unwrap();
    let production_id_c = indexed.productions_for_label(label_c).next().unwrap();
    let label_d = *label_map.get("D").unwrap();
    let production_id_d = indexed.productions_for_label(label_d).next().unwrap();
    expect_that!(
      closure(ProductionPosition::new(production_id_a, 1), &indexed).collect_vec(),
      unordered_elements_are![
        &ProductionPosition::new(production_id_a, 1),
        &ProductionPosition::new(production_id_b, 0),
        &ProductionPosition::new(production_id_c, 0),
        &ProductionPosition::new(production_id_d, 0),
      ]
    );
  }
}
