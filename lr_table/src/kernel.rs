use std::{
  fmt::{Debug, Display},
  hash::Hash,
};

use itertools::Itertools;

use crate::position::Position;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Kernel {
  /// The positions in this kernel, sorted according to
  /// `PositionWithOrdering::Ord`. This is required for `Hash` and `Eq` to
  /// work correctly on this struct.
  positions: Vec<Position>,
}

impl Kernel {
  pub fn new(mut positions: Vec<Position>) -> Self {
    positions.sort_by_key(Position::position);
    Self { positions }
  }

  pub fn positions(&self) -> impl Iterator<Item = &Position> {
    self.positions.iter()
  }
}

impl Display for Kernel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{{{}}}",
      self.positions.iter().map(|pos| format!("{pos}")).join(", ")
    )
  }
}
