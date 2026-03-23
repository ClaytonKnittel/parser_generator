use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Clone)]
pub struct ParserError {
  message: String,
}

impl ParserError {
  pub fn new(message: impl Into<String>) -> Self {
    Self {
      message: message.into(),
    }
  }
}

impl Error for ParserError {}

impl Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type ParserResult<T = ()> = Result<T, ParserError>;
