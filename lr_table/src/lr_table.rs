use std::hash::Hash;

use itertools::Itertools;

use crate::{
  closure::closure_follow_sets, first_map::FirstTable, grammar::Grammar,
  indexed_grammar::IndexedGrammar, kernel::Kernel, kernel_table::KernelTable, position::Position,
  vocabulary::Vocabulary,
};

#[derive(Clone, Copy)]
pub struct StateId(usize);

impl StateId {
  pub fn new(id: usize) -> Self {
    Self(id)
  }

  pub fn id(&self) -> usize {
    self.0
  }
}

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
  fn generate_actions<T: Vocabulary>(indexed_grammar: &IndexedGrammar<T>) {
    let first_set = FirstTable::build_from_grammar(indexed_grammar);
    let mut kernel_table = KernelTable::<T>::new();

    let root_label = indexed_grammar.root_production_label();
    let initial_kernel = Kernel::new(
      indexed_grammar
        .production_rule_ids_for_label(root_label)
        .map(|rule_id| Position::new_top_level(rule_id))
        .collect(),
    );
    let id = kernel_table.get_or_insert(initial_kernel);
    debug_assert_eq!(id.id(), 0);

    for state_id in 0.. {
      let state_id = StateId::new(state_id);
      let Some(kernel) = kernel_table.get_state(state_id) else {
        break;
      };

      let follow_sets = closure_follow_sets(kernel, indexed_grammar, &first_set)
        .into_iter()
        .collect_vec();
    }
  }

  pub fn build<T: Clone, L: Clone + Eq + Hash>(grammar: &Grammar<T, L>) -> Self {
    let indexed_grammar = IndexedGrammar::build(grammar);
    todo!();
  }
}
