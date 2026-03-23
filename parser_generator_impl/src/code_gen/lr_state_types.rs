use std::marker::PhantomData;

use itertools::Itertools;
use lr_table::lr_table::LRTable;

use crate::type_symbol::Type;

pub enum LRStateType {
  Reduce(Option<Type>),
  Terminal,
}

pub struct LRStateTypes<'a> {
  types: Vec<LRStateType>,
  /// This lookup table is lifetime-bound to an instance of `LRTable`.
  _phantom: PhantomData<&'a ()>,
}

impl<'a> LRStateTypes<'a> {
  pub fn build_from_lr_table<T>(lr_table: &'a LRTable<T>) -> Self {
    let mut types = (0..lr_table.num_states())
      .map(|_| LRStateType::Terminal)
      .collect_vec();

    // for

    Self {
      types,
      _phantom: PhantomData,
    }
  }
}
