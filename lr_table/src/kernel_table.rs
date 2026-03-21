use std::{
  collections::{HashMap, hash_map::Entry},
  rc::Rc,
};

use crate::{kernel::Kernel, lr_table::StateId};

pub struct KernelTable {
  table: HashMap<Rc<Kernel>, StateId>,
  states: Vec<Rc<Kernel>>,
}

impl KernelTable {
  pub fn new() -> Self {
    Self {
      table: HashMap::new(),
      states: Vec::new(),
    }
  }

  pub fn get_state(&self, state_id: StateId) -> Option<&Kernel> {
    self.states.get(state_id.id()).map(|state| state.as_ref())
  }

  pub fn get_or_insert(&mut self, kernel: Kernel) -> StateId {
    let table_size = self.table.len();
    let kernel = Rc::new(kernel);
    match self.table.entry(kernel.clone()) {
      Entry::Occupied(entry) => *entry.get(),
      Entry::Vacant(entry) => {
        self.states.push(kernel);

        let state_id = StateId::new(table_size);
        entry.insert(state_id);
        state_id
      }
    }
  }
}
