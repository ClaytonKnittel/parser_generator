use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Clone, Debug)]
pub enum BuildGrammarError {
  EmptyGrammar,
  RootProductionRepeated,
  RootProductionReferenced,
  NotConnected,
}

impl Display for BuildGrammarError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EmptyGrammar => write!(f, "grammar is empty"),
      Self::RootProductionRepeated => write!(f, "the root production has multiple rules"),
      Self::RootProductionReferenced => {
        write!(f, "the root production is referenced by another rule")
      }
      Self::NotConnected => write!(f, "The grammar is not fully connected"),
    }
  }
}

macro_rules! grammar_error {
  ($error:ident) => {
    $crate::error::LRTableError::build_grammar($crate::error::BuildGrammarError::$error)
  };
}

pub(crate) use grammar_error;

#[derive(Clone)]
pub enum LRTableError {
  BuildGrammar(BuildGrammarError),
  LabelAlreadyExists { label_id: usize },
  Generic(String),
}

impl LRTableError {
  pub fn build_grammar(error: BuildGrammarError) -> Self {
    Self::BuildGrammar(error)
  }

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
      Self::BuildGrammar(error) => write!(f, "Build grammar error: {error}"),
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
