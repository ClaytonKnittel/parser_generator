use std::{fmt::Debug, marker::PhantomData};

use itertools::Itertools;

use crate::bit_set::BitSet;

pub trait Label: Clone {
  fn id(&self) -> usize;
  fn from_id(id: usize) -> Self;
}

impl Label for u8 {
  fn id(&self) -> usize {
    *self as usize
  }

  fn from_id(id: usize) -> u8 {
    debug_assert!(id < u8::MAX as usize);
    id as u8
  }
}

impl Label for usize {
  fn id(&self) -> usize {
    *self
  }

  fn from_id(id: usize) -> Self {
    id
  }
}

impl<L: Label> Label for Option<L> {
  fn id(&self) -> usize {
    match self {
      Some(label) => label.id() + 1,
      None => 0,
    }
  }

  fn from_id(id: usize) -> Self {
    if id == 0 {
      None
    } else {
      Some(L::from_id(id - 1))
    }
  }
}

pub struct FixedSizeSet<L> {
  set: BitSet,
  _phantom: PhantomData<L>,
}

impl<L: Label> FixedSizeSet<L> {
  pub fn new(capacity: usize) -> Self {
    Self {
      set: BitSet::new(capacity),
      _phantom: PhantomData,
    }
  }

  pub fn has(&self, label: &L) -> bool {
    self.set.has(label.id())
  }

  pub fn set(&mut self, label: &L) {
    self.set.set(label.id())
  }

  pub fn full(&self) -> bool {
    self.set.full()
  }
}

pub struct FixedSizeMap<L, T> {
  map: Vec<T>,
  _phantom: PhantomData<L>,
}

impl<L: Label, T: Default> FixedSizeMap<L, T> {
  pub fn new(capacity: usize) -> Self {
    Self::new_with_constructor(capacity, T::default)
  }
}

impl<L: Label, T> FixedSizeMap<L, T> {
  pub fn new_with_constructor<F>(capacity: usize, mut constructor: F) -> Self
  where
    F: FnMut() -> T,
  {
    Self {
      map: (0..capacity).map(move |_| constructor()).collect(),
      _phantom: PhantomData,
    }
  }
}

impl<L: Label, T> FixedSizeMap<L, T> {
  pub fn get(&self, label: &L) -> &T {
    &self.map[label.id()]
  }

  pub fn get_mut(&mut self, label: &L) -> &mut T {
    &mut self.map[label.id()]
  }
}

impl<L: Debug + Label, T: Debug> Debug for FixedSizeMap<L, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..self.map.len())
        .map(L::from_id)
        .map(|label| { format!("{label:?}: {:?}", self.get(&label)) })
        .join(", ")
    )
  }
}

#[derive(Clone)]
struct SparseFixedSizedEntry<L, T> {
  /// Replicate the label here for use in `iter()` / `into_iter()`.
  label: L,
  value: T,
}

#[derive(Clone)]
pub struct SparseFixedSizeMap<L, T> {
  index_map: Vec<usize>,
  map: Vec<SparseFixedSizedEntry<L, T>>,
  _phantom: PhantomData<L>,
}

impl<L: Label, T> SparseFixedSizeMap<L, T> {
  const UNINITIALIZED_INDEX: usize = usize::MAX;

  pub fn new(capacity: usize) -> Self {
    Self {
      index_map: vec![Self::UNINITIALIZED_INDEX; capacity],
      map: Vec::new(),
      _phantom: PhantomData,
    }
  }

  fn maybe_index(&self, label: &L) -> Option<usize> {
    match self.index_map[label.id()] {
      Self::UNINITIALIZED_INDEX => None,
      index => Some(index),
    }
  }

  /// Returns an optional reference to the value for the given label. Returns
  /// `None` if the label is not in the map.
  pub fn get(&self, label: &L) -> Option<&T> {
    self.maybe_index(label).map(|index| &self.map[index].value)
  }

  /// Returns an optional mutable reference to the value for the given label.
  /// Returns `None` if the label is not in the map.
  pub fn get_mut(&mut self, label: &L) -> Option<&mut T> {
    self
      .maybe_index(label)
      .map(|index| &mut self.map[index].value)
  }

  fn insert(&mut self, label: &L, value: T) -> &mut T {
    debug_assert_eq!(self.index_map[label.id()], Self::UNINITIALIZED_INDEX);

    let index = self.map.len();
    self.index_map[label.id()] = index;
    self.map.push(SparseFixedSizedEntry {
      label: label.clone(),
      value,
    });
    &mut self.map[index].value
  }

  pub fn try_insert(&mut self, label: &L, value: T) -> Result<(), &T> {
    match self.maybe_index(label) {
      Some(index) => Err(&self.map[index].value),
      None => {
        self.insert(label, value);
        Ok(())
      }
    }
  }

  pub fn get_mut_or_insert_with<F>(&mut self, label: &L, construct: F) -> &mut T
  where
    F: FnOnce() -> T,
  {
    match self.maybe_index(label) {
      Some(index) => &mut self.map[index].value,
      None => self.insert(label, construct()),
    }
  }

  /// Either returns a mutable reference to the value with given label, or
  /// inserts a new one with default value.
  pub fn get_mut_or_default(&mut self, label: &L) -> &mut T
  where
    T: Default,
  {
    self.get_mut_or_insert_with(label, || T::default())
  }

  /// Returns an iterator over all initialized entries in the map.
  pub fn iter(&self) -> impl Iterator<Item = (L, &T)> {
    self
      .map
      .iter()
      .map(|entry| (entry.label.clone(), &entry.value))
  }
}

impl<L: Label + 'static, T: 'static> IntoIterator for SparseFixedSizeMap<L, T> {
  type Item = (L, T);
  type IntoIter = Box<dyn Iterator<Item = (L, T)>>;

  fn into_iter(self) -> Self::IntoIter {
    Box::new(self.map.into_iter().map(|entry| (entry.label, entry.value)))
  }
}

impl<L: Debug + Label, T: Debug> Debug for SparseFixedSizeMap<L, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      self
        .iter()
        .map(|(label, value)| { format!("{label:?}: {value:?}") })
        .join(", ")
    )
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::fixed_map::SparseFixedSizeMap;

  #[gtest]
  fn test_sparse_iter_empty() {
    let sparse = SparseFixedSizeMap::<usize, usize>::new(10);
    expect_that!(sparse.iter().collect_vec(), is_empty());
  }

  #[gtest]
  fn test_sparse_into_iter_empty() {
    let sparse = SparseFixedSizeMap::<usize, usize>::new(10);
    expect_that!(sparse.into_iter().collect_vec(), is_empty());
  }

  #[gtest]
  fn test_sparse_iter_one() {
    let mut sparse = SparseFixedSizeMap::<usize, usize>::new(10);
    expect_eq!(*sparse.get_mut_or_insert_with(&5, || 1000), 1000);
    expect_that!(sparse.iter().collect_vec(), elements_are![&(5, &1000)]);
  }

  #[gtest]
  fn test_sparse_into_iter_one() {
    let mut sparse = SparseFixedSizeMap::<usize, usize>::new(10);
    expect_eq!(*sparse.get_mut_or_insert_with(&5, || 1000), 1000);
    expect_that!(sparse.into_iter().collect_vec(), elements_are![&(5, 1000)]);
  }

  #[gtest]
  fn test_sparse_iter_full() {
    let mut sparse = SparseFixedSizeMap::<usize, usize>::new(10);
    for i in 0..10 {
      expect_eq!(*sparse.get_mut_or_insert_with(&i, || 1000 + i), 1000 + i);
    }
    expect_that!(
      sparse.iter().collect_vec(),
      unordered_elements_are![
        &(0, &1000),
        &(1, &1001),
        &(2, &1002),
        &(3, &1003),
        &(4, &1004),
        &(5, &1005),
        &(6, &1006),
        &(7, &1007),
        &(8, &1008),
        &(9, &1009),
      ]
    );
  }

  #[gtest]
  fn test_sparse_into_iter_full() {
    let mut sparse = SparseFixedSizeMap::<usize, usize>::new(10);
    for i in 0..10 {
      expect_eq!(*sparse.get_mut_or_insert_with(&i, || 1000 + i), 1000 + i);
    }
    expect_that!(
      sparse.into_iter().collect_vec(),
      unordered_elements_are![
        &(0, 1000),
        &(1, 1001),
        &(2, 1002),
        &(3, 1003),
        &(4, 1004),
        &(5, 1005),
        &(6, 1006),
        &(7, 1007),
        &(8, 1008),
        &(9, 1009),
      ]
    );
  }
}
