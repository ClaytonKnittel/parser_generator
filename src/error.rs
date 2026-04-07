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
  UserError(E),
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

  pub fn from_user_error<F: Into<E>>(err: F) -> Self {
    Self::UserError(err.into())
  }
}

impl<E: Error> From<E> for ParserError<E> {
  fn from(value: E) -> Self {
    Self::from_user_error(value)
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
      Self::UserError(err) => write!(f, "{err}"),
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
      Self::UserError(err) => write!(f, "{err:?}"),
    }
  }
}

pub type ParserResult<T, E> = Result<T, ParserError<E>>;
