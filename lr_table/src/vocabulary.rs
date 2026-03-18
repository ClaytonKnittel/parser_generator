use std::fmt::{Debug, Display};

use crate::fixed_map::Label;

pub trait Vocabulary: Copy + Eq {
  /// The size of the vocabulary.
  const SIZE: usize;

  /// Returns a unique integer value in the range 0..Self::SIZE for each
  /// element of the vocabulary.
  fn ordinal(&self) -> usize;

  /// Turns an ordinal back into `Self`. The inverse of `Self::ordinal()`.
  fn from_ordinal(ordinal: usize) -> Self;
}

impl Vocabulary for u8 {
  const SIZE: usize = u8::MAX as usize + 1;

  fn ordinal(&self) -> usize {
    *self as usize
  }

  fn from_ordinal(ordinal: usize) -> Self {
    ordinal as u8
  }
}

impl<T: Vocabulary> Label for T {
  fn id(self) -> usize {
    self.ordinal()
  }

  fn from_id(id: usize) -> Self {
    debug_assert!(id < Self::SIZE);
    Self::from_ordinal(id)
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AugmentedVocab<T> {
  Token(T),
  Epsilon,
  EndOfStream,
}

impl<T: Vocabulary> Vocabulary for AugmentedVocab<T> {
  const SIZE: usize = T::SIZE + 2;

  fn ordinal(&self) -> usize {
    match self {
      Self::Token(t) => t.ordinal() + 2,
      Self::Epsilon => 0,
      Self::EndOfStream => 1,
    }
  }

  fn from_ordinal(ordinal: usize) -> Self {
    if ordinal == 0 {
      Self::Epsilon
    } else if ordinal == 1 {
      Self::EndOfStream
    } else {
      Self::Token(T::from_ordinal(ordinal - 2))
    }
  }
}

impl<T> From<T> for AugmentedVocab<T> {
  fn from(value: T) -> Self {
    Self::Token(value)
  }
}

impl<T: Display> Display for AugmentedVocab<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Token(t) => write!(f, "{t}"),
      Self::Epsilon => write!(f, "ε"),
      Self::EndOfStream => write!(f, "$"),
    }
  }
}

impl<T: Debug> Debug for AugmentedVocab<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Token(t) => write!(f, "{t:?}"),
      Self::Epsilon => write!(f, "ε"),
      Self::EndOfStream => write!(f, "$"),
    }
  }
}
