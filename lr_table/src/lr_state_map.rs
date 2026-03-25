use std::{
  collections::{HashMap, hash_map::Entry},
  marker::PhantomData,
};

use cknittel_util::iter::CollectResult;
use itertools::Itertools;

use crate::{
  error::{LRTableError, LRTableResult},
  indexed_grammar::{IndexedGrammar, ProductionLabel},
  lr_table::{Action, LRTable, StateId},
};

fn insert_state_and_type(
  state: StateId,
  ty: LRStateType,
  map: &mut HashMap<StateId, LRStateType>,
) -> LRTableResult {
  match map.entry(state) {
    Entry::Occupied(entry) => entry.get().verify_compatible(ty),
    Entry::Vacant(entry) => {
      entry.insert(ty);
      Ok(())
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum LRStateType {
  Reduce(ProductionLabel),
  Terminal,
  Root,
}

impl LRStateType {
  fn verify_compatible(&self, other: LRStateType) -> LRTableResult {
    match (&self, other) {
      (Self::Reduce(state1), LRStateType::Reduce(state2)) => {
        if *state1 != state2 {
          return Err(LRTableError::state_resolve_conflict());
        }
      }
      (Self::Terminal, LRStateType::Terminal) => {}
      (Self::Root, LRStateType::Root) => {}
      _ => {
        return Err(LRTableError::state_resolve_conflict());
      }
    }

    Ok(())
  }
}

struct LRStateInfo {
  /// A list of states that may precede this state in parsing.
  prev_states: Vec<StateId>,
  /// The return type of states that may precede this state.
  prev_state_return_type: LRStateType,
}

impl LRStateInfo {
  fn build_from_state_map(map: HashMap<StateId, LRStateType>) -> LRTableResult<Self> {
    let mut iter = map.into_iter();
    let (state1, return_type) = iter.next().ok_or_else(LRTableError::unresolved_states)?;
    let mut prev_states = vec![state1];

    for (state, state_return_type) in iter {
      return_type.verify_compatible(state_return_type)?;
      prev_states.push(state);
    }

    Ok(Self {
      prev_states,
      prev_state_return_type: return_type,
    })
  }
}

/// A map from states to the set of states that may immediately precede them
/// in parsing.
///
/// This map is also useful when determining the type that should be held in
/// each state's enum variant.
///
/// Each state may be preceded by either the consumption of a terminal (i.e. it
/// is the target of a shift action), or the resolution of a production rule
/// (i.e. it is the target of a GOTO action).
pub struct LRStateMap<'a> {
  /// A map from StateId -> state info.
  state_map: Vec<LRStateInfo>,
  /// This lookup table is lifetime-bound to an instance of `LRTable`.
  _phantom: PhantomData<&'a ()>,
}

impl<'a> LRStateMap<'a> {
  pub fn build_from_lr_table<T: Clone, L>(
    grammar: &IndexedGrammar<T, L>,
    lr_table: &'a LRTable<T>,
  ) -> LRTableResult<Self> {
    let mut state_map = (0..lr_table.num_states())
      .map(|_| HashMap::<StateId, LRStateType>::new())
      .collect_vec();

    // Set the root production rule's type to `Root`.
    state_map[0].insert(lr_table.root_state(), LRStateType::Root);

    for state in lr_table.states() {
      for (_, action) in lr_table.state_actions(state, grammar) {
        if let Action::Shift { next_state } = action {
          insert_state_and_type(
            state,
            LRStateType::Terminal,
            &mut state_map[next_state.id()],
          )?;
        }
        // Reduce / accept actions don't have immediate targets.
      }

      for (from_rule, action) in lr_table.goto_actions(state, grammar) {
        insert_state_and_type(
          state,
          LRStateType::Reduce(from_rule),
          &mut state_map[action.state().id()],
        )?;
      }
    }

    debug_assert_eq!(state_map.len(), lr_table.num_states());
    let state_map = state_map
      .into_iter()
      .map(LRStateInfo::build_from_state_map)
      .collect_result_vec()?;

    Ok(Self {
      state_map,
      _phantom: PhantomData,
    })
  }

  pub fn state_type(&self, state: StateId) -> LRStateType {
    self.state_map[state.id()].prev_state_return_type
  }

  /// Given a state, returns an iterator over all states that may immediately
  /// precede it on the parser stack.
  pub fn back_edges(&self, state: StateId) -> impl Iterator<Item = StateId> {
    self.state_map[state.id()].prev_states.iter().cloned()
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
