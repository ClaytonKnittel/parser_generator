use std::{
  error::Error,
  fmt::{Debug, Display},
};

pub enum LRTableError {
  LabelAlreadyExists { label_id: usize },
  Generic(String),
}

impl LRTableError {
  pub fn label_already_exists(label_id: usize) -> Self {
    Self::LabelAlreadyExists { label_id }
  }

  pub fn new_generic(message: String) -> Self {
    LRTableError::Generic(message)
  }
}

impl Error for LRTableError {}

impl Display for LRTableError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::LabelAlreadyExists { label_id } => write!(f, "Label {label_id} already exists"),
      Self::Generic(message) => write!(f, "Error: {}", message),
    }
  }
}

impl Debug for LRTableError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type LRTableResult<T = ()> = Result<T, LRTableError>;
