use std::fmt::{Debug, Display};

use crate::fixed_map::Label;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AugmentedVocabToken<T> {
  Token(T),
  Epsilon,
  EndOfStream,
}

impl<T: Label> Label for AugmentedVocabToken<T> {
  fn id(&self) -> usize {
    match self {
      AugmentedVocabToken::Token(token) => token.id() + 2,
      AugmentedVocabToken::Epsilon => 0,
      AugmentedVocabToken::EndOfStream => 1,
    }
  }

  fn from_id(ordinal: usize) -> Self {
    if ordinal == 0 {
      AugmentedVocabToken::Epsilon
    } else if ordinal == 1 {
      AugmentedVocabToken::EndOfStream
    } else {
      AugmentedVocabToken::Token(T::from_id(ordinal - 2))
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
