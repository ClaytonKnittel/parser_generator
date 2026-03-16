pub trait Vocabulary: Sized {
  /// The size of the vocabulary.
  const SIZE: usize;

  /// Returns a unique integer value in the range 0..Self::SIZE for each
  /// element of the vocabulary.
  fn ordinal(&self) -> usize;
}

impl Vocabulary for u8 {
  const SIZE: usize = u8::MAX as usize + 1;

  fn ordinal(&self) -> usize {
    *self as usize
  }
}

#[derive(Clone, Copy, Debug)]
pub enum AugmentedVocab<T> {
  Token(T),
  Epsilon,
  EndOfStream,
}

impl<T: Vocabulary> AugmentedVocab<T> {
  /// The size of the augmented vocabulary, including epsilon and EOF.
  pub const SIZE: usize = T::SIZE + 2;
}
