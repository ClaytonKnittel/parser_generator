use std::{fmt::Debug, marker::PhantomData};

use itertools::Itertools;

use crate::{
  bit_set::BitSet,
  error::{LRTableError, LRTableResult},
};

pub trait Label: Copy {
  fn id(self) -> usize;
  fn from_id(id: usize) -> Self;
}

impl<L: Label> Label for Option<L> {
  fn id(self) -> usize {
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

  pub fn get(&self, label: L) -> bool {
    self.set.get(label.id())
  }

  pub fn set(&mut self, label: L) {
    self.set.set(label.id());
  }
}

pub struct FixedSizeMap<L, T> {
  map: Vec<T>,
  _phantom: PhantomData<L>,
}

impl<L: Label, T: Default> FixedSizeMap<L, T> {
  pub fn new(capacity: usize) -> Self {
    Self {
      map: (0..capacity).map(|_| T::default()).collect(),
      _phantom: PhantomData,
    }
  }
}

impl<L: Label, T> FixedSizeMap<L, T> {
  pub fn get(&self, label: L) -> &T {
    &self.map[label.id()]
  }

  pub fn get_mut(&mut self, label: L) -> &mut T {
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
        .map(|label| { format!("{label:?}: {:?}", self.get(label)) })
        .join(", ")
    )
  }
}

pub struct SparseFixedSizeMap<L, T> {
  index_map: Vec<usize>,
  map: Vec<T>,
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

  fn maybe_index(&self, label: L) -> Option<usize> {
    match self.index_map[label.id()] {
      Self::UNINITIALIZED_INDEX => None,
      index => Some(index),
    }
  }

  fn index(&self, label: L) -> usize {
    self.maybe_index(label).unwrap()
  }

  /// Returns an optional reference to the value for the given label. Returns
  /// `None` if the label is not in the map.
  pub fn get(&self, label: L) -> Option<&T> {
    self.maybe_index(label).map(|index| &self.map[index])
  }

  /// Returns an optional mutable reference to the value for the given label.
  /// Returns `None` if the label is not in the map.
  pub fn get_mut(&mut self, label: L) -> Option<&mut T> {
    self.maybe_index(label).map(|index| &mut self.map[index])
  }

  fn insert(&mut self, label: L, value: T) -> &mut T {
    debug_assert_eq!(self.index_map[label.id()], Self::UNINITIALIZED_INDEX);

    let next_index = self.map.len();
    self.index_map[label.id()] = next_index;
    self.map.push(value);
    &mut self.map[next_index]
  }

  pub fn try_insert(&mut self, label: L, value: T) -> LRTableResult {
    if self.maybe_index(label).is_some() {
      Err(LRTableError::label_already_exists(label.id()))
    } else {
      self.insert(label, value);
      Ok(())
    }
  }

  pub fn get_mut_or_insert_with<F>(&mut self, label: L, construct: F) -> &mut T
  where
    F: FnOnce() -> T,
  {
    match self.maybe_index(label) {
      Some(index) => &mut self.map[index],
      None => self.insert(label, construct()),
    }
  }

  /// Inserts a new value into the sparse map, returning a mutable reference to
  /// the newly inserted value. If the value already existed, returns `None`
  /// and does not modify the map.
  pub fn maybe_insert(&mut self, label: L, value: T) -> Option<&mut T> {
    if self.index_map[label.id()] != Self::UNINITIALIZED_INDEX {
      None
    } else {
      Some(self.insert(label, value))
    }
  }

  /// Either returns a mutable reference to the value with given label, or
  /// inserts a new one with default value.
  pub fn get_mut_or_default(&mut self, label: L) -> &mut T
  where
    T: Default,
  {
    self.get_mut_or_insert_with(label, || T::default())
  }

  /// Inserts a new default-initialized value into the sparse map, returning a
  /// mutable reference to the newly inserted value. If the value already
  /// existed, returns `None` and does not modify the map.
  pub fn maybe_insert_default(&mut self, label: L) -> Option<&mut T>
  where
    T: Default,
  {
    if self.index_map[label.id()] != Self::UNINITIALIZED_INDEX {
      None
    } else {
      Some(self.insert(label, T::default()))
    }
  }

  /// Returns an iterator over all initialized entries in the map.
  pub fn iter(&self) -> impl Iterator<Item = (L, &T)> {
    (0..self.index_map.len())
      .map(Label::from_id)
      .filter_map(|label| {
        self
          .maybe_index(label)
          .map(|index| (label, &self.map[index]))
      })
  }

  fn take_label(&mut self, label: L) -> Option<(L, T)>
  where
    T: Default,
  {
    self.maybe_index(label).map(|index| {
      let mut tmp = T::default();
      std::mem::swap(&mut self.map[index], &mut tmp);
      (label, tmp)
    })
  }
}

pub struct SparseFixedSizeMapIntoIter<L, T> {
  sparse_map: SparseFixedSizeMap<L, T>,
  label_id: usize,
}

impl<L: Label, T: Default> Iterator for SparseFixedSizeMapIntoIter<L, T> {
  type Item = (L, T);

  fn next(&mut self) -> Option<Self::Item> {
    let label = Label::from_id(self.label_id);
    self.sparse_map.maybe_index(label).map(|index| {
      self.label_id += 1;

      let mut tmp = T::default();
      std::mem::swap(&mut self.sparse_map.map[index], &mut tmp);
      (label, tmp)
    })
  }
}

impl<L: Label, T: Default> IntoIterator for SparseFixedSizeMap<L, T> {
  type Item = (L, T);
  type IntoIter = SparseFixedSizeMapIntoIter<L, T>;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      sparse_map: self,
      label_id: 0,
    }
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
