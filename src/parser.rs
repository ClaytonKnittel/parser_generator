use std::borrow::Borrow;

use crate::error::ParserResult;

pub trait Parser {
  type Token;
  type Value;

  fn parse<I, B>(input_stream: I) -> ParserResult<Self::Value>
  where
    I: IntoIterator<Item = B>,
    B: Borrow<Self::Token>;
}
