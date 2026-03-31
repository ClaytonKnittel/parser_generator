use std::{
  error::Error,
  fmt::{Debug, Display},
};

use proc_macro_error::abort;
use proc_macro2::Span;

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

  pub fn from_foreign_error(error: impl Error, span: Span) -> Self {
    Self::new(format!("{error}"), span)
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

pub trait InterceptResult {
  /// Appents the given string to the error in this result. This is a no-op if
  /// the result is `Ok`.
  fn intercept(self, message: impl Display) -> Self;
}

impl<T> InterceptResult for ParserGeneratorResult<T> {
  fn intercept(self, message: impl Display) -> Self {
    self.map_err(|mut err| {
      err.message = format!("{}: {}", err.message, message);
      err
    })
  }
}
