use std::fmt::Display;

pub trait Vocabulary: Sized {
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

#[derive(Clone, Copy)]
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

impl<T: Display> Display for AugmentedVocab<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Token(t) => write!(f, "{t}"),
      Self::Epsilon => write!(f, "ε"),
      Self::EndOfStream => write!(f, "$"),
    }
  }
}
