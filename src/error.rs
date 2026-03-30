use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Clone)]
pub enum ParserError<E> {
  ParseError { message: String },
  InputStreamError(E),
}

impl<E> ParserError<E> {
  pub fn new(message: impl Into<String>) -> Self {
    Self::ParseError {
      message: message.into(),
    }
  }

  pub fn from_input_stream_error(err: E) -> Self {
    Self::InputStreamError(err)
  }
}

impl<E: Error> Error for ParserError<E> {}

impl<E: Display> Display for ParserError<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError { message } => write!(f, "{message}"),
      Self::InputStreamError(err) => write!(f, "{err}"),
    }
  }
}

impl<E: Debug> Debug for ParserError<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError { message } => write!(f, "{message}"),
      Self::InputStreamError(err) => write!(f, "{err:?}"),
    }
  }
}

pub type ParserResult<T, E> = Result<T, ParserError<E>>;
