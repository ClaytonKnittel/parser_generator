use std::marker::PhantomData;

use itertools::Itertools;

use crate::{
  error::{LRTableError, LRTableResult},
  indexed_grammar::{IndexedGrammar, ProductionLabel},
  lr_table::{Action, LRTable, StateId},
};

#[derive(Clone, Copy, Debug)]
pub enum LRStateType {
  Reduce(ProductionLabel),
  Terminal,
  Root,
}

#[derive(Default)]
enum LRStateTypeBuilder {
  Reduce(ProductionLabel),
  Terminal,
  Root,
  #[default]
  Unknown,
}

impl LRStateTypeBuilder {
  fn finalize(self) -> LRTableResult<LRStateType> {
    match self {
      Self::Reduce(production_label) => Ok(LRStateType::Reduce(production_label)),
      Self::Terminal => Ok(LRStateType::Terminal),
      Self::Root => Ok(LRStateType::Root),
      Self::Unknown => Err(LRTableError::unresolved_states()),
    }
  }

  fn set(&mut self, value: LRStateType) -> LRTableResult {
    match (&self, value) {
      (Self::Reduce(state1), LRStateType::Reduce(state2)) => {
        if *state1 != state2 {
          return Err(LRTableError::state_resolve_conflict());
        }
      }
      (Self::Terminal, LRStateType::Terminal) => {}
      (Self::Root, LRStateType::Root) => {}
      (Self::Unknown, LRStateType::Reduce(maybe_type)) => *self = Self::Reduce(maybe_type),
      (Self::Unknown, LRStateType::Terminal) => *self = Self::Terminal,
      (Self::Unknown, LRStateType::Root) => *self = Self::Root,
      _ => {
        return Err(LRTableError::state_resolve_conflict());
      }
    }

    Ok(())
  }
}

/// A map from states to the type of action that will immediately precede them
/// in parsing. This map is useful when determining the type that should be
/// held in each state's enum variant.
///
/// Each state may be preceded by either the consumption of a terminal (i.e. it
/// is the target of a shift action), or the resolution of a production rule
/// (i.e. it is the target of a GOTO action).
pub struct LRStateMap<'a> {
  types: Vec<LRStateType>,
  /// This lookup table is lifetime-bound to an instance of `LRTable`.
  _phantom: PhantomData<&'a ()>,
}

impl<'a> LRStateMap<'a> {
  pub fn build_from_lr_table<T: Clone, L>(
    grammar: &IndexedGrammar<T, L>,
    lr_table: &'a LRTable<T>,
  ) -> LRTableResult<Self> {
    let mut types = (0..lr_table.num_states())
      .map(|_| LRStateTypeBuilder::default())
      .collect_vec();

    // Set the root production rule's type to `Root`.
    types[0].set(LRStateType::Root)?;

    for state in lr_table.states() {
      for (_, action) in lr_table.state_actions(state, grammar) {
        if let Action::Shift { next_state } = action {
          types[next_state.id()].set(LRStateType::Terminal)?;
        }
        // Reduce / accept actions don't have immediate targets.
      }

      for (from_rule, action) in lr_table.goto_actions(state, grammar) {
        types[action.state().id()].set(LRStateType::Reduce(from_rule))?;
      }
    }

    let types = types
      .into_iter()
      .map(LRStateTypeBuilder::finalize)
      .collect::<Result<_, _>>()?;

    Ok(Self {
      types,
      _phantom: PhantomData,
    })
  }

  pub fn state_type(&self, state: StateId) -> LRStateType {
    self.types[state.id()]
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    grammar::Grammar,
    indexed_grammar::IndexedGrammar,
    lr_state_map::{LRStateMap, LRStateType},
    lr_table::LRTable,
  };
  use googletest::prelude::*;
  use itertools::Itertools;

  #[gtest]
  fn test_trivial() {
    let grammar = Grammar::from_grammar_str("A -> a").unwrap();

    let grammar = IndexedGrammar::build(&grammar).unwrap();
    let lr_table = LRTable::build(&grammar).unwrap();
    let state_table = LRStateMap::build_from_lr_table(&grammar, &lr_table).unwrap();

    let root_state = lr_table.states().next().unwrap();

    expect_that!(state_table.state_type(root_state), pat![LRStateType::Root]);
  }

  #[gtest]
  fn test_single_rule() {
    let grammar = Grammar::from_grammar_str("A -> a b").unwrap();

    let grammar = IndexedGrammar::build(&grammar).unwrap();
    let lr_table = LRTable::build(&grammar).unwrap();
    let state_table = LRStateMap::build_from_lr_table(&grammar, &lr_table).unwrap();

    let states = lr_table.states().collect_vec();
    expect_that!(state_table.state_type(states[0]), pat![LRStateType::Root]);
    expect_that!(
      state_table.state_type(states[1]),
      pat![LRStateType::Terminal]
    );
  }

  #[gtest]
  fn test_two_rules() {
    let grammar = Grammar::from_grammar_str(
      r#"A -> B
         B -> a"#,
    )
    .unwrap();

    let (grammar, label_map) = IndexedGrammar::build_with_label_map(&grammar).unwrap();
    let label_b = *label_map.get("B").unwrap();
    let lr_table = LRTable::build(&grammar).unwrap();
    let state_table = LRStateMap::build_from_lr_table(&grammar, &lr_table).unwrap();

    let states = lr_table.states().collect_vec();
    expect_that!(state_table.state_type(states[0]), pat![LRStateType::Root]);
    expect_that!(
      state_table.state_type(states[1]),
      pat![LRStateType::Reduce(label_b)]
    );
  }
}
