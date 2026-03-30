use std::{
  borrow::Borrow,
  error::Error,
  fmt::{Debug, Display},
};

use crate::error::ParserResult;

#[derive(Clone)]
pub enum Infallible {}

impl Debug for Infallible {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {}
  }
}
impl Display for Infallible {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {}
  }
}

impl Error for Infallible {}

pub trait Parser {
  type Token;
  type Value;

  fn parse_fallible<I, B, E>(input_stream: I) -> ParserResult<Self::Value, E>
  where
    I: IntoIterator<Item = Result<B, E>>,
    B: Borrow<Self::Token>,
    E: Clone;

  fn parse<I, B>(input_stream: I) -> ParserResult<Self::Value, Infallible>
  where
    I: IntoIterator<Item = B>,
    B: Borrow<Self::Token>,
  {
    Self::parse_fallible(input_stream.into_iter().map(|v| Ok(v)))
  }
}
