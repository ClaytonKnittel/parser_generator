use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LRTableError {
  message: String,
}

impl LRTableError {
  pub fn new(message: String) -> Self {
    LRTableError { message }
  }
}

impl Error for LRTableError {}

impl Display for LRTableError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error: {}", self.message)
  }
}

pub type LRTableResult<T = ()> = Result<T, Box<dyn Error + Send + Sync + 'static>>;
