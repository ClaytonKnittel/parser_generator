use std::{
  collections::{HashMap, hash_map::Entry},
  fmt::{Debug, Display},
  hash::Hash,
};

use itertools::Itertools;

use crate::{augmented_vocab_token::AugmentedVocabToken, bit_set::BitSet, fixed_map::Label};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenId(usize);

impl Label for TokenId {
  fn id(&self) -> usize {
    self.0
  }

  fn from_id(id: usize) -> Self {
    Self(id)
  }
}

pub type AugmentedTokenId = AugmentedVocabToken<TokenId>;

pub struct VocabularyBuilder<T> {
  token_map: HashMap<T, TokenId>,
  id_map: Vec<T>,
}

impl<T: Clone + Eq + Hash> VocabularyBuilder<T> {
  pub fn get_id_or_insert(&mut self, token: AugmentedVocabToken<T>) -> TokenId {
    match token {
      AugmentedVocabToken::Token(token) => {
        let next_id = TokenId(self.token_map.len());
        match self.token_map.entry(token.clone()) {
          Entry::Occupied(entry) => *entry.get(),
          Entry::Vacant(entry) => {
            self.id_map.push(token);
            entry.insert(next_id);
            next_id
          }
        }
      }
      AugmentedVocabToken::Epsilon => {
        TokenId::from_id(AugmentedVocabToken::<TokenId>::Epsilon.id())
      }
      AugmentedVocabToken::EndOfStream => {
        TokenId::from_id(AugmentedVocabToken::<TokenId>::EndOfStream.id())
      }
    }
  }
}

impl<T> VocabularyBuilder<T> {
  pub fn new() -> Self {
    Self {
      token_map: HashMap::new(),
      id_map: Vec::new(),
    }
  }

  pub fn build(self) -> AugmentedVocab<T> {
    AugmentedVocab {
      token_map: self.token_map,
      id_map: self.id_map,
    }
  }
}

impl<T> Default for VocabularyBuilder<T> {
  fn default() -> Self {
    Self::new()
  }
}

pub struct AugmentedVocab<T> {
  token_map: HashMap<T, TokenId>,
  id_map: Vec<T>,
}

impl<T> AugmentedVocab<T> {
  pub fn size(&self) -> usize {
    self.id_map.len() + 2
  }

  pub fn for_each_id(&self) -> impl Iterator<Item = AugmentedTokenId> {
    (0..self.size()).map(AugmentedVocabToken::from_id)
  }
}

impl<T: Eq + Hash> AugmentedVocab<T> {
  pub fn token_to_id(&self, token: &T) -> TokenId {
    *self.token_map.get(token).unwrap()
  }

  pub fn augmented_token_to_id(&self, token: &AugmentedVocabToken<T>) -> AugmentedTokenId {
    match token {
      AugmentedVocabToken::Token(token) => self.token_to_id(token).into(),
      AugmentedVocabToken::Epsilon => AugmentedVocabToken::Epsilon,
      AugmentedVocabToken::EndOfStream => AugmentedVocabToken::EndOfStream,
    }
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VocabSet {
  set: BitSet,
}

impl VocabSet {
  pub fn new<T>(vocab: &AugmentedVocab<T>) -> Self {
    Self {
      set: BitSet::new(vocab.size()),
    }
  }

  pub fn has(&self, token: AugmentedVocabToken<TokenId>) -> bool {
    self.set.has(token.id())
  }

  pub fn set(&mut self, token: AugmentedVocabToken<TokenId>) {
    self.set.set(token.id());
  }

  pub fn clear(&mut self, token: AugmentedVocabToken<TokenId>) {
    self.set.clear(token.id());
  }

  /// Merges `other` into self, adding each entry of `other` which was not
  /// already in `self`. Returns true if `self` changed.
  pub fn merge(&mut self, other: &Self) -> bool {
    self.set.merge(&other.set)
  }

  pub fn iter(&self) -> impl Iterator<Item = AugmentedVocabToken<TokenId>> {
    self.set.for_each().map(AugmentedVocabToken::from_id)
  }
}

#[cfg(test)]
impl VocabSet {
  pub fn from_iter<T: Eq + Hash>(
    iter: impl IntoIterator<Item = AugmentedVocabToken<T>>,
    vocab: &AugmentedVocab<T>,
  ) -> Self {
    let mut s = Self::new(vocab);
    for token in iter.into_iter() {
      s.set(vocab.augmented_token_to_id(&token));
    }
    s
  }
}

impl Display for VocabSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.set.for_each().join("/"))
  }
}

impl Debug for VocabSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.set.for_each().join("/"))
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::{
    augmented_vocab_token::AugmentedVocabToken,
    fixed_map::Label,
    vocabulary::{AugmentedTokenId, TokenId, VocabularyBuilder},
  };

  #[gtest]
  fn test_empty_vocab() {
    let vocab = VocabularyBuilder::<u8>::new().build();
    expect_eq!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::Epsilon),
      AugmentedTokenId::Epsilon
    );
    expect_eq!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::EndOfStream),
      AugmentedTokenId::EndOfStream
    );

    expect_that!(
      vocab.for_each_id().collect_vec(),
      unordered_elements_are![
        &AugmentedVocabToken::Epsilon,
        &AugmentedVocabToken::EndOfStream
      ]
    );

    expect_that!(
      vocab.for_each_id().map(|id| id.id()).collect_vec(),
      unordered_elements_are![&0, &1]
    );
  }

  #[gtest]
  fn test_single_token_vocab() {
    let mut builder = VocabularyBuilder::new();
    let a_id = builder.get_id_or_insert(b'a'.into());
    expect_eq!(a_id, TokenId(0));

    let vocab = builder.build();
    expect_eq!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::Epsilon),
      AugmentedTokenId::Epsilon
    );
    expect_eq!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::EndOfStream),
      AugmentedTokenId::EndOfStream
    );
    expect_eq!(vocab.augmented_token_to_id(&b'a'.into()), a_id.into());
    expect_eq!(vocab.token_to_id(&b'a'), a_id);

    expect_that!(
      vocab.for_each_id().collect_vec(),
      unordered_elements_are![
        &AugmentedVocabToken::Epsilon,
        &AugmentedVocabToken::EndOfStream,
        &AugmentedVocabToken::Token(a_id)
      ]
    );

    expect_that!(
      vocab.for_each_id().map(|id| id.id()).collect_vec(),
      unordered_elements_are![&0, &1, &2]
    );
  }

  #[gtest]
  fn test_three_token_vocab() {
    let mut builder = VocabularyBuilder::new();
    let a_id = builder.get_id_or_insert(b'a'.into());
    let b_id = builder.get_id_or_insert(b'b'.into());
    let c_id = builder.get_id_or_insert(b'c'.into());
    expect_eq!(a_id, TokenId(0));
    expect_eq!(b_id, TokenId(1));
    expect_eq!(c_id, TokenId(2));

    let vocab = builder.build();
    expect_eq!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::Epsilon),
      AugmentedTokenId::Epsilon
    );
    expect_eq!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::EndOfStream),
      AugmentedTokenId::EndOfStream
    );
    expect_eq!(vocab.augmented_token_to_id(&b'a'.into()), a_id.into());
    expect_eq!(vocab.token_to_id(&b'a'), a_id);
    expect_eq!(vocab.augmented_token_to_id(&b'b'.into()), b_id.into());
    expect_eq!(vocab.token_to_id(&b'b'), b_id);
    expect_eq!(vocab.augmented_token_to_id(&b'c'.into()), c_id.into());
    expect_eq!(vocab.token_to_id(&b'c'), c_id);

    expect_that!(
      vocab.for_each_id().collect_vec(),
      unordered_elements_are![
        &AugmentedVocabToken::Epsilon,
        &AugmentedVocabToken::EndOfStream,
        &AugmentedVocabToken::Token(a_id),
        &AugmentedVocabToken::Token(b_id),
        &AugmentedVocabToken::Token(c_id)
      ]
    );

    expect_that!(
      vocab.for_each_id().map(|id| id.id()).collect_vec(),
      unordered_elements_are![&0, &1, &2, &3, &4]
    );
  }

  #[gtest]
  fn test_label_id_roundtrip() {
    let mut builder = VocabularyBuilder::new();
    builder.get_id_or_insert(b'a'.into());
    builder.get_id_or_insert(b'b'.into());
    builder.get_id_or_insert(b'c'.into());
    builder.get_id_or_insert(b'd'.into());
    builder.get_id_or_insert(b'e'.into());

    let vocab = builder.build();
    expect_that!(
      vocab
        .for_each_id()
        .map(|id| id.id())
        .map(AugmentedVocabToken::from_id)
        .collect_vec(),
      container_eq(vocab.for_each_id().collect_vec())
    );
  }

  #[gtest]
  fn test_insert_id_twice() {
    let mut builder = VocabularyBuilder::new();
    let a_id = builder.get_id_or_insert(b'a'.into());
    let b_id = builder.get_id_or_insert(b'b'.into());

    expect_eq!(builder.get_id_or_insert(b'b'.into()), b_id);
    expect_eq!(builder.get_id_or_insert(b'a'.into()), a_id);
  }
}
