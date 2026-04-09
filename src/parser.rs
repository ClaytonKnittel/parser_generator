use std::{borrow::Borrow, convert::Infallible, fmt::Debug};

use crate::error::{ParserResult, ParserUserError, ParserUserErrorOrInfallible};

pub trait Parser {
  type Token: Clone + Debug;
  type Value;
  type Error: ParserUserError + Clone;

  fn parse_fallible<I, B, E>(
    input_stream: I,
  ) -> ParserResult<Self::Value, Self::Token, Self::Error>
  where
    I: IntoIterator<Item = Result<B, E>>,
    B: Borrow<Self::Token>,
    E: ParserUserErrorOrInfallible<Self::Token, Self::Error> + Clone;

  fn parse<I, B>(input_stream: I) -> ParserResult<Self::Value, Self::Token, Self::Error>
  where
    I: IntoIterator<Item = B>,
    B: Borrow<Self::Token>,
  {
    Self::parse_fallible(input_stream.into_iter().map(|v| Ok::<_, Infallible>(v)))
  }
}
