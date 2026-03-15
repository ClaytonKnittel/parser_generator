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
  action_table: Vec<Action>,
  goto_table: Vec<GotoAction>,
}

impl LRTable {
  pub fn build<T: Clone, L: Clone + Eq + Hash>(grammar: &Grammar<T, L>) -> Self {
    let indexed_grammar = IndexedGrammar::build(grammar);
    todo!();
  }
}
