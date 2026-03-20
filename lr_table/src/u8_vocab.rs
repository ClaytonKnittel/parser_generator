use crate::vocabulary::{Vocabulary, VocabularyToken};

pub struct U8Vocab;

impl VocabularyToken for u8 {
  type Vocab = U8Vocab;
}

impl Vocabulary for U8Vocab {
  type Token = u8;

  fn size(&self) -> usize {
    u8::MAX as usize + 1
  }

  fn ordinal(token: &Self::Token) -> usize {
    *token as usize
  }

  fn from_ordinal(ordinal: usize) -> Self::Token {
    debug_assert!(ordinal <= u8::MAX as usize);
    ordinal as u8
  }
}
