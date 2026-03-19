use std::{
  fmt::{Debug, Display},
  hash::{Hash, Hasher},
};

use itertools::Itertools;

use crate::{position::Position, vocabulary::Vocabulary};

pub struct Kernel<T> {
  /// The positions in this kernel, sorted according to
  /// `PositionWithOrdering::Ord`. This is required for `Hash` and `Eq` to
  /// work correctly on this struct.
  positions: Vec<Position<T>>,
}

impl<T> Kernel<T> {
  pub fn new(mut positions: Vec<Position<T>>) -> Self {
    positions.sort_by_key(Position::position);
    Self { positions }
  }

  pub fn positions(&self) -> impl Iterator<Item = &Position<T>> {
    self.positions.iter()
  }
}

impl<T> PartialEq for Kernel<T> {
  fn eq(&self, other: &Self) -> bool {
    self.positions.eq(&other.positions)
  }
}

impl<T> Eq for Kernel<T> {}

impl<T> Hash for Kernel<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.positions.hash(state);
  }
}

impl<T: Debug + Vocabulary> Debug for Kernel<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.positions)
  }
}

impl<T: Display + Vocabulary> Display for Kernel<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{{{}}}",
      self.positions.iter().map(|pos| format!("{pos}")).join(", ")
    )
  }
}
