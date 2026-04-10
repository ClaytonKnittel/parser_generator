use std::{
  convert::Infallible,
  error::Error,
  fmt::{Debug, Display},
};

pub trait ParserUserError: Error + From<Infallible> + Clone {}

pub trait ParserUserErrorOrInfallible<T, E>: Error {
  fn into_user_error(self) -> ParserError<T, E>;
}

impl<T, E: ParserUserError, F: Into<E> + Error> ParserUserErrorOrInfallible<T, E> for F {
  fn into_user_error(self) -> ParserError<T, E> {
    ParserError::UserError(self.into())
  }
}

#[derive(Clone, Debug)]
pub struct NoUserErrorType;
impl Error for NoUserErrorType {}
impl Display for NoUserErrorType {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(())
  }
}
impl From<Infallible> for NoUserErrorType {
  fn from(value: Infallible) -> Self {
    match value {}
  }
}
impl ParserUserError for NoUserErrorType {}

#[derive(Clone)]
pub enum ParserError<T, E> {
  ParseError {
    next_token: Option<T>,
  },
  #[cfg(debug_assertions)]
  OverlappingTokenMatchers {
    token: String,
  },
  UserError(E),
}

impl<T, E> ParserError<T, E> {
  pub fn new(next_token: Option<T>) -> Self {
    Self::ParseError { next_token }
  }

  #[cfg(debug_assertions)]
  pub fn overlapping_token_matchers(token: String) -> Self {
    Self::OverlappingTokenMatchers { token }
  }
}

impl<T, E: ParserUserError> From<E> for ParserError<T, E> {
  fn from(value: E) -> Self {
    Self::UserError(value)
  }
}

impl<T, E: Error> From<Infallible> for ParserError<T, E> {
  fn from(value: Infallible) -> Self {
    match value {}
  }
}

impl<T: Debug, E: Error> Error for ParserError<T, E> {}

impl<T: Debug, E: Display> Display for ParserError<T, E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError { next_token } => write!(f, "Failed to parse: unexpected {next_token:?}"),
      #[cfg(debug_assertions)]
      Self::OverlappingTokenMatchers { token } => write!(
        f,
        "Token {token} matches multiple rules. Disambiguate matchers for tokens of this type."
      ),
      Self::UserError(err) => write!(f, "{err}"),
    }
  }
}

impl<T: Debug, E: Debug> Debug for ParserError<T, E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError { next_token } => write!(f, "Failed to parse: unexpected {next_token:?}"),
      #[cfg(debug_assertions)]
      Self::OverlappingTokenMatchers { token } => write!(
        f,
        "Token {token} matches multiple rules. Disambiguate matchers for tokens of this type."
      ),
      Self::UserError(err) => write!(f, "{err:?}"),
    }
  }
}

pub type ParserResult<T, TokenT, E> = Result<T, ParserError<TokenT, E>>;
