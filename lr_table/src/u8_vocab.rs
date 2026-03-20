use crate::vocabulary::{Vocabulary, VocabularyToken};

#[derive(Default)]
pub struct U8Vocab;

impl VocabularyToken for u8 {
  type Vocab = U8Vocab;

  fn ordinal(&self) -> usize {
    *self as usize
  }

  fn from_ordinal(ordinal: usize) -> Self {
    debug_assert!(ordinal <= u8::MAX as usize);
    ordinal as u8
  }
}

impl Vocabulary for U8Vocab {
  type Token = u8;

  fn size(&self) -> usize {
    u8::MAX as usize + 1
  }
}
