use std::fmt::{Debug, Display};

pub trait VocabularyToken: Clone + Eq {
  type Vocab: Vocabulary;
}

pub trait Vocabulary {
  type Token: VocabularyToken;

  /// Returns the size of the vocabulary.
  fn size(&self) -> usize;

  /// Returns a unique integer value in the range 0..Self::size() for each
  /// element of the vocabulary.
  fn ordinal(token: &Self::Token) -> usize;

  /// Turns an ordinal back into `Self`. The inverse of `Self::ordinal()`.
  fn from_ordinal(ordinal: usize) -> Self::Token;

  /// Returns an iterator over all tokens in this vocabulary.
  fn for_each(&self) -> impl Iterator<Item = Self::Token> {
    (0..self.size()).map(Self::from_ordinal)
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AugmentedVocabToken<T> {
  Token(T),
  Epsilon,
  EndOfStream,
}

impl<T: VocabularyToken> VocabularyToken for AugmentedVocab<T> {
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
