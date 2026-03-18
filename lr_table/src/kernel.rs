use std::{
  cmp::Ordering,
  fmt::{Debug, Display},
  hash::{Hash, Hasher},
};

use itertools::Itertools;

use crate::{position::Position, vocabulary::Vocabulary};

/// Implements `Ord` and `Hash` for `Position`. The follow set is ignored
/// because all positions in the kernel are guaranteed to be unique under
/// (production rule, position).
struct PositionWithOrdering<T>(Position<T>);

impl<T> PartialEq for PositionWithOrdering<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0.position().eq(&other.0.position())
  }
}

impl<T> Eq for PositionWithOrdering<T> {}

impl<T> PartialOrd for PositionWithOrdering<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for PositionWithOrdering<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.position().cmp(&other.0.position())
  }
}

impl<T> Hash for PositionWithOrdering<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.position().hash(state);
  }
}

impl<T: Display + Vocabulary> Debug for PositionWithOrdering<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

impl<T: Display + Vocabulary> Display for PositionWithOrdering<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub struct Kernel<T> {
  /// The positions in this kernel, sorted according to
  /// `PositionWithOrdering::Ord`. This is required for `Hash` and `Eq` to
  /// work correctly on this struct.
  positions: Vec<PositionWithOrdering<T>>,
}

impl<T> Kernel<T> {
  pub fn new(positions: Vec<Position<T>>) -> Self {
    let mut positions = positions
      .into_iter()
      .map(PositionWithOrdering)
      .collect_vec();
    positions.sort();
    Self { positions }
  }

  pub fn positions(&self) -> impl Iterator<Item = &Position<T>> {
    self
      .positions
      .iter()
      .map(|PositionWithOrdering(position)| position)
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

impl<T: Display + Vocabulary> Debug for Kernel<T> {
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
