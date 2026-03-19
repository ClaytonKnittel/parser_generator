use std::{
  error::Error,
  fmt::{Debug, Display},
};

use proc_macro::Span;
use proc_macro_error::abort;

#[derive(Clone)]
pub struct ParserGeneratorError {
  message: String,
  span: Span,
}

impl ParserGeneratorError {
  pub fn new(message: impl Into<String>, span: Span) -> Self {
    Self {
      message: message.into(),
      span,
    }
  }

  pub fn abort(&self) -> ! {
    abort!(self.span, self.message)
  }
}

impl Error for ParserGeneratorError {}

impl Display for ParserGeneratorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for ParserGeneratorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type ParserGeneratorResult<T = ()> = Result<T, ParserGeneratorError>;
