use std::{borrow::Borrow, fmt::Debug};

use crate::error::ParserResult;

pub trait Parser {
  type Token: Debug;
  type Value;
  type Error: std::error::Error + Clone;

  fn parse_fallible<I, B, E>(input_stream: I) -> ParserResult<Self::Value, Self::Error>
  where
    I: IntoIterator<Item = Result<B, E>>,
    B: Borrow<Self::Token>,
    E: Into<Self::Error> + Clone;

  fn parse<I, B>(input_stream: I) -> ParserResult<Self::Value, Self::Error>
  where
    I: IntoIterator<Item = B>,
    B: Borrow<Self::Token>,
  {
    Self::parse_fallible(input_stream.into_iter().map(|v| Ok::<_, Self::Error>(v)))
  }
}
