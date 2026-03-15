use crate::grammar::Grammar;

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
  pub fn build<T, L>(grammar: &Grammar<T, L>) -> Self {
    todo!();
  }
}
