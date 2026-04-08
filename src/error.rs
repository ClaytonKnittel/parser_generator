use std::{
  convert::Infallible,
  error::Error,
  fmt::{Debug, Display},
};

pub trait ParserUserError: Error + From<Infallible> + Clone {}

pub trait ParserUserErrorOrInfallible<E>: Error {
  fn into_user_error(self) -> ParserError<E>;
}

impl<E: ParserUserError, F: Into<E> + Error> ParserUserErrorOrInfallible<E> for F {
  fn into_user_error(self) -> ParserError<E> {
    ParserError::UserError(self.into())
  }
}

// impl<E: ParserUserError> ParserUserErrorOrInfallible<E> for Infallible {
//   fn into_user_error(self) -> ParserError<E> {
//     match self {}
//   }
// }

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
}

impl<E: ParserUserError> From<E> for ParserError<E> {
  fn from(value: E) -> Self {
    Self::UserError(value)
  }
}

impl<E: Error> From<Infallible> for ParserError<E> {
  fn from(value: Infallible) -> Self {
    match value {}
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
