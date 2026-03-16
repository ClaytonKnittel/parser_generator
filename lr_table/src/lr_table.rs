use std::hash::Hash;

use crate::{grammar::Grammar, indexed_grammar::IndexedGrammar};

struct StateId(usize);

enum Action {
  Shift { next_state: StateId },
  Reduce { next_state: StateId },
  Accept,
}

struct GotoAction(StateId);

pub struct LRTable {
  /// A vocab_size * num_states sized table of actions.
  action_table: Vec<Action>,
  /// A num_production_labels * num_states sized table of goto actions.
  goto_table: Vec<GotoAction>,
}

impl LRTable {
  pub fn build<T: Clone, L: Clone + Eq + Hash>(grammar: &Grammar<T, L>) -> Self {
    let indexed_grammar = IndexedGrammar::build(grammar);
    todo!();
  }
}
