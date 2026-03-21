use std::{
  collections::{HashMap, hash_map::Entry},
  fmt::{Debug, Display},
  hash::Hash,
};

use itertools::Itertools;

use crate::{
  bit_set::BitSet,
  error::{LRTableError, LRTableResult},
  fixed_map::Label,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenId(usize);

impl Display for TokenId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub(crate) type AugmentedTokenId = AugmentedVocabToken<TokenId>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AugmentedVocabToken<T> {
  Token(T),
  Epsilon,
  EndOfStream,
}

impl Label for AugmentedVocabToken<TokenId> {
  fn id(&self) -> usize {
    match self {
      AugmentedVocabToken::Token(token) => token.0 + 2,
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
      AugmentedVocabToken::Token(TokenId(ordinal - 2))
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

pub(crate) struct VocabularyBuilder<T> {
  token_map: HashMap<T, TokenId>,
  id_map: Vec<T>,
}

impl<T: Clone + Eq + Hash> VocabularyBuilder<T> {
  pub fn get_id_or_insert(&mut self, token: AugmentedVocabToken<T>) -> AugmentedTokenId {
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
        .into()
      }
      AugmentedVocabToken::Epsilon => AugmentedTokenId::Epsilon,
      AugmentedVocabToken::EndOfStream => AugmentedTokenId::EndOfStream,
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

impl<T: Eq + Hash + ToString> AugmentedVocab<T> {
  pub(crate) fn token_to_id(&self, token: &T) -> LRTableResult<TokenId> {
    self
      .token_map
      .get(token)
      .ok_or_else(|| LRTableError::UnrecognizedToken {
        token: token.to_string(),
      })
      .cloned()
  }

  pub(crate) fn augmented_token_to_id(
    &self,
    token: &AugmentedVocabToken<T>,
  ) -> LRTableResult<AugmentedTokenId> {
    Ok(match token {
      AugmentedVocabToken::Token(token) => self.token_to_id(token)?.into(),
      AugmentedVocabToken::Epsilon => AugmentedVocabToken::Epsilon,
      AugmentedVocabToken::EndOfStream => AugmentedVocabToken::EndOfStream,
    })
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct VocabSet {
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
  pub fn from_iter<T: Eq + Hash + ToString>(
    iter: impl IntoIterator<Item = AugmentedVocabToken<T>>,
    vocab: &AugmentedVocab<T>,
  ) -> Self {
    let mut s = Self::new(vocab);
    for token in iter.into_iter() {
      s.set(vocab.augmented_token_to_id(&token).unwrap());
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
    fixed_map::Label,
    vocabulary::{AugmentedTokenId, AugmentedVocabToken, TokenId, VocabularyBuilder},
  };

  #[gtest]
  fn test_empty_vocab() {
    let vocab = VocabularyBuilder::<u8>::new().build();
    expect_eq!(vocab.size(), 2);
    expect_that!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::Epsilon),
      ok(eq(&AugmentedTokenId::Epsilon))
    );
    expect_that!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::EndOfStream),
      ok(eq(&AugmentedTokenId::EndOfStream))
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
    expect_eq!(a_id, TokenId(0).into());

    let vocab = builder.build();
    expect_eq!(vocab.size(), 3);
    expect_that!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::Epsilon),
      ok(eq(&AugmentedTokenId::Epsilon))
    );
    expect_that!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::EndOfStream),
      ok(eq(&AugmentedTokenId::EndOfStream))
    );
    expect_that!(vocab.augmented_token_to_id(&b'a'.into()), ok(eq(&a_id)));
    expect_that!(
      vocab.token_to_id(&b'a').map(AugmentedTokenId::from),
      ok(eq(&a_id))
    );

    expect_that!(
      vocab.for_each_id().collect_vec(),
      unordered_elements_are![
        &AugmentedVocabToken::Epsilon,
        &AugmentedVocabToken::EndOfStream,
        &a_id
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
    expect_eq!(a_id, TokenId(0).into());
    expect_eq!(b_id, TokenId(1).into());
    expect_eq!(c_id, TokenId(2).into());

    let vocab = builder.build();
    expect_eq!(vocab.size(), 5);
    expect_that!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::Epsilon),
      ok(eq(&AugmentedTokenId::Epsilon))
    );
    expect_that!(
      vocab.augmented_token_to_id(&AugmentedVocabToken::EndOfStream),
      ok(eq(&AugmentedTokenId::EndOfStream))
    );
    expect_that!(vocab.augmented_token_to_id(&b'a'.into()), ok(eq(&a_id)));
    expect_that!(
      vocab.token_to_id(&b'a').map(AugmentedTokenId::from),
      ok(eq(&a_id))
    );
    expect_that!(vocab.augmented_token_to_id(&b'b'.into()), ok(eq(&b_id)));
    expect_that!(
      vocab.token_to_id(&b'b').map(AugmentedTokenId::from),
      ok(eq(&b_id))
    );
    expect_that!(vocab.augmented_token_to_id(&b'c'.into()), ok(eq(&c_id)));
    expect_that!(
      vocab.token_to_id(&b'c').map(AugmentedTokenId::from),
      ok(eq(&c_id))
    );

    expect_that!(
      vocab.for_each_id().collect_vec(),
      unordered_elements_are![
        &AugmentedVocabToken::Epsilon,
        &AugmentedVocabToken::EndOfStream,
        &a_id,
        &b_id,
        &c_id
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
    expect_eq!(vocab.size(), 7);
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

    let vocab = builder.build();
    expect_eq!(vocab.size(), 4);
  }

  #[gtest]
  fn test_insert_epsilon_and_eof() {
    let mut builder = VocabularyBuilder::new();
    let epsilon_id = builder.get_id_or_insert(AugmentedVocabToken::Epsilon);
    let eof_id = builder.get_id_or_insert(AugmentedVocabToken::EndOfStream);

    expect_eq!(epsilon_id, AugmentedTokenId::Epsilon);
    expect_eq!(eof_id, AugmentedTokenId::EndOfStream);

    let a_id = builder.get_id_or_insert(b'a'.into());
    expect_ne!(a_id, epsilon_id);
    expect_ne!(a_id, eof_id);

    let vocab = builder.build();
    expect_eq!(vocab.size(), 3);
  }
}
