use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Clone)]
pub enum ParserError<E> {
  ParseError {
    message: String,
  },
  #[cfg(debug_assertions)]
  OverlappingTokenMatchers {
    token: String,
  },
  InputStreamError(E),
  ForeignError {
    message: String,
  },
}

impl<E> ParserError<E> {
  pub fn new(message: impl Into<String>) -> Self {
    Self::ParseError {
      message: message.into(),
    }
  }

  #[cfg(debug_assertions)]
  pub fn overlapping_token_matchers(token: String) -> Self {
    Self::OverlappingTokenMatchers { token }
  }

  pub fn from_input_stream_error(err: E) -> Self {
    Self::InputStreamError(err)
  }

  pub fn from_foreign_error<F: Error>(err: F) -> Self {
    Self::ForeignError {
      message: err.to_string(),
    }
  }
}

impl<E: Error> Error for ParserError<E> {}

impl<E: Display> Display for ParserError<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError { message } => write!(f, "{message}"),
      #[cfg(debug_assertions)]
      Self::OverlappingTokenMatchers { token } => write!(
        f,
        "Token {token} matches multiple rules. Disambiguate matchers for tokens of this type."
      ),
      Self::InputStreamError(err) => write!(f, "{err}"),
      Self::ForeignError { message } => write!(f, "Foreign error: {message}"),
    }
  }
}

impl<E: Debug> Debug for ParserError<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError { message } => write!(f, "{message}"),
      #[cfg(debug_assertions)]
      Self::OverlappingTokenMatchers { token } => write!(
        f,
        "Token {token} matches multiple rules. Disambiguate matchers for tokens of this type."
      ),
      Self::InputStreamError(err) => write!(f, "{err:?}"),
      Self::ForeignError { message } => write!(f, "Foreign error: {message}"),
    }
  }
}

pub type ParserResult<T, E> = Result<T, ParserError<E>>;
