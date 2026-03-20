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
  ordinal_map: HashMap<T, TokenId>,
  id_map: Vec<T>,
}

impl<T: Clone + Eq + Hash> VocabularyBuilder<T> {
  pub fn get_id_or_insert(&mut self, token: AugmentedVocabToken<T>) -> TokenId {
    match token {
      AugmentedVocabToken::Token(token) => {
        let next_id = TokenId(self.ordinal_map.len());
        match self.ordinal_map.entry(token.clone()) {
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
      ordinal_map: HashMap::new(),
      id_map: Vec::new(),
    }
  }

  pub fn build(self) -> AugmentedVocab<T> {
    AugmentedVocab {
      ordinal_map: self.ordinal_map,
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
  ordinal_map: HashMap<T, TokenId>,
  id_map: Vec<T>,
}

impl<T> AugmentedVocab<T> {
  pub fn size(&self) -> usize {
    self.id_map.len() + 2
  }
}

impl<T: Clone> AugmentedVocab<T> {
  pub fn id_to_token(&self, id: TokenId) -> T {
    debug_assert!(id.0 >= 2);
    self.id_map[id.0 - 2].clone()
  }

  pub fn for_each_id(&self) -> impl Iterator<Item = AugmentedTokenId> {
    (0..self.size()).map(AugmentedVocabToken::from_id)
  }

  pub fn for_each(&self) -> impl Iterator<Item = T> {
    self.id_map.iter().cloned()
  }
}

impl<T: Eq + Hash> AugmentedVocab<T> {
  pub fn token_to_id(&self, token: &T) -> TokenId {
    *self.ordinal_map.get(token).unwrap()
  }

  pub fn augmented_token_to_id(&self, token: &AugmentedVocabToken<T>) -> AugmentedTokenId {
    match token {
      AugmentedVocabToken::Token(token) => self.token_to_id(token).into(),
      AugmentedVocabToken::Epsilon => AugmentedVocabToken::Epsilon,
      AugmentedVocabToken::EndOfStream => AugmentedVocabToken::EndOfStream,
    }
  }
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
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
