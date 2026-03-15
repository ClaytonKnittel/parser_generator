use crate::{
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionRuleId},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProductionPosition {
  production_id: ProductionRuleId,
  position: usize,
}

fn closure<T>(
  position: ProductionPosition,
  grammar: &IndexedGrammar<T>,
) -> impl Iterator<Item = ProductionPosition> {
  let mut positions = Vec::new();
  let mut stack = vec![position];
  while let Some(pos) = stack.pop() {
    positions.push(pos);
    let production = grammar.production_rule(pos.production_id);
    if pos.position >= production.rule().len() {
      continue;
    }

    if let ProductionNode::Production(label) = &production.rule()[pos.position] {
      for production_id in grammar.productions_for_label(*label) {
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
    indexed_grammar::{IndexedGrammar, ProductionLabel},
    table_builder::{ProductionPosition, closure},
  };

  #[gtest]
  fn test_small_closure() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a"#,
    )
    .unwrap();

    let indexed = IndexedGrammar::build(&grammar);
    let production_id_a = indexed
      .productions_for_label(ProductionLabel::new(0))
      .next()
      .unwrap();
    let production_id_b = indexed
      .productions_for_label(ProductionLabel::new(1))
      .next()
      .unwrap();
    expect_that!(
      closure(
        ProductionPosition {
          production_id: production_id_a,
          position: 0
        },
        &indexed
      )
      .collect_vec(),
      unordered_elements_are![
        &ProductionPosition {
          production_id: production_id_a,
          position: 0
        },
        &ProductionPosition {
          production_id: production_id_b,
          position: 0
        }
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

    let indexed = IndexedGrammar::build(&grammar);
    let production_id_a = indexed
      .productions_for_label(ProductionLabel::new(0))
      .next()
      .unwrap();
    let production_id_b = indexed
      .productions_for_label(ProductionLabel::new(1))
      .next()
      .unwrap();
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
        &ProductionPosition {
          production_id: production_id_a,
          position: 1
        },
        &ProductionPosition {
          production_id: production_id_b,
          position: 0
        }
      ]
    );
  }
}
