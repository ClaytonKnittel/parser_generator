use std::{
  fmt::{Debug, Display},
  marker::PhantomData,
};

use itertools::Itertools;

use crate::{bit_set::BitSet, vocabulary::Vocabulary};

pub struct VocabSet<T> {
  set: BitSet,
  _phantom: PhantomData<T>,
}

impl<T: Vocabulary> VocabSet<T> {
  pub fn new() -> Self {
    Self {
      set: BitSet::new(T::SIZE),
      _phantom: PhantomData,
    }
  }

  pub fn get(&self, value: &T) -> bool {
    self.set.get(value.ordinal())
  }

  pub fn set(&mut self, value: &T) {
    self.set.set(value.ordinal());
  }

  pub fn clear(&mut self, value: &T) {
    self.set.clear(value.ordinal());
  }

  /// Merges `other` into self, adding each entry of `other` which was not
  /// already in `self`. Returns true if `self` changed.
  pub fn merge(&mut self, other: &Self) -> bool {
    self.set.merge(&other.set)
  }

  pub fn iter(&self) -> impl Iterator<Item = T> {
    self.set.for_each().map(T::from_ordinal)
  }
}

#[cfg(test)]
impl<T: Vocabulary> VocabSet<T> {
  pub fn from_iter<U>(iter: impl IntoIterator<Item = U>) -> Self
  where
    U: Into<T>,
  {
    let mut s = Self::new();
    for token in iter.into_iter() {
      s.set(&token.into());
    }
    s
  }
}

impl<T: Vocabulary> Default for VocabSet<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> PartialEq for VocabSet<T> {
  fn eq(&self, other: &Self) -> bool {
    self.set == other.set
  }
}

impl<T> Eq for VocabSet<T> {}

impl<T> Clone for VocabSet<T> {
  fn clone(&self) -> Self {
    Self {
      set: self.set.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<T: Display + Vocabulary> Display for VocabSet<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.set.for_each().map(T::from_ordinal).join("/"))
  }
}

impl<T: Debug + Vocabulary> Debug for VocabSet<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self
        .set
        .for_each()
        .map(T::from_ordinal)
        .map(|token| format!("{token:?}"))
        .join("/")
    )
  }
}
