use std::fmt::{Debug, Display};

use crate::fixed_map::Label;

pub trait VocabularyToken: Clone + Eq {
  type Vocab: Vocabulary<Token = Self>;

  /// Returns a unique integer value in the range 0..Self::size() for each
  /// element of the vocabulary.
  fn ordinal(&self) -> usize;

  /// Turns an ordinal back into `Self`. The inverse of `Self::ordinal()`.
  fn from_ordinal(ordinal: usize) -> Self;
}

impl<T: VocabularyToken + Clone + Eq> Label for T {
  fn id(&self) -> usize {
    self.ordinal()
  }

  fn from_id(id: usize) -> Self {
    Self::from_ordinal(id)
  }
}

pub trait Vocabulary {
  type Token: VocabularyToken<Vocab = Self>;

  /// Returns the size of the vocabulary.
  fn size(&self) -> usize;

  /// Returns an iterator over all tokens in this vocabulary.
  fn for_each(&self) -> impl Iterator<Item = Self::Token> {
    (0..self.size()).map(Self::Token::from_ordinal)
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AugmentedVocabToken<T> {
  Token(T),
  Epsilon,
  EndOfStream,
}

impl<T: VocabularyToken> VocabularyToken for AugmentedVocabToken<T> {
  type Vocab = AugmentedVocab<T::Vocab>;

  fn ordinal(&self) -> usize {
    match self {
      AugmentedVocabToken::Token(token) => T::ordinal(token) + 2,
      AugmentedVocabToken::Epsilon => 0,
      AugmentedVocabToken::EndOfStream => 1,
    }
  }

  fn from_ordinal(ordinal: usize) -> Self {
    if ordinal == 0 {
      AugmentedVocabToken::Epsilon
    } else if ordinal == 1 {
      AugmentedVocabToken::EndOfStream
    } else {
      AugmentedVocabToken::Token(T::from_ordinal(ordinal - 2))
    }
  }
}

impl<T> From<T> for AugmentedVocabToken<T> {
  fn from(value: T) -> Self {
    Self::Token(value)
  }
}

impl<T: Display> Display for AugmentedVocabToken<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Token(t) => write!(f, "{t}"),
      Self::Epsilon => write!(f, "ε"),
      Self::EndOfStream => write!(f, "$"),
    }
  }
}

impl<T: Debug> Debug for AugmentedVocabToken<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Token(t) => write!(f, "{t:?}"),
      Self::Epsilon => write!(f, "ε"),
      Self::EndOfStream => write!(f, "$"),
    }
  }
}

#[derive(Default)]
pub struct AugmentedVocab<T> {
  vocab: T,
}

impl<T: Vocabulary> Vocabulary for AugmentedVocab<T> {
  type Token = AugmentedVocabToken<T::Token>;

  fn size(&self) -> usize {
    self.vocab.size() + 2
  }
}

impl<T> From<T> for AugmentedVocab<T> {
  fn from(value: T) -> Self {
    Self { vocab: value }
  }
}
