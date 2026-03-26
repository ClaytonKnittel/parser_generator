use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Clone, Debug)]
pub enum BuildGrammarError {
  EmptyGrammar,
  RootProductionRepeated,
  RootProductionReferenced,
  NotConnected(String),
}

impl Display for BuildGrammarError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EmptyGrammar => write!(f, "grammar is empty"),
      Self::RootProductionRepeated => write!(f, "the root production has multiple rules"),
      Self::RootProductionReferenced => {
        write!(f, "the root production is referenced by another rule")
      }
      Self::NotConnected(message) => write!(f, "The grammar is not fully connected: {message}"),
    }
  }
}

macro_rules! grammar_error {
  ($error:ident) => {
    $crate::error::LRTableError::build_grammar($crate::error::BuildGrammarError::$error)
  };
  ($error:ident$(, $args:expr)+) => {
    $crate::error::LRTableError::build_grammar($crate::error::BuildGrammarError::$error($($args.into(),)+))
  };
}

pub(crate) use grammar_error;

use crate::indexed_grammar::ProductionRuleId;

#[derive(Clone)]
pub enum LRTableError {
  BuildGrammar(BuildGrammarError),
  UnrecognizedToken {
    token: String,
  },
  LabelAlreadyExists {
    label_id: usize,
  },
  ShiftReduceConflict {
    rule: ProductionRuleId,
    lookahead: String,
  },
  ReduceConflict {
    rule1: ProductionRuleId,
    rule2: ProductionRuleId,
    lookahead: String,
  },
  UnresolvedStates,
  StateResolveConflict,
  Generic(String),
}

impl LRTableError {
  pub fn build_grammar(error: BuildGrammarError) -> Self {
    Self::BuildGrammar(error)
  }

  pub fn label_already_exists(label_id: usize) -> Self {
    Self::LabelAlreadyExists { label_id }
  }

  pub fn shift_reduce_conflict(rule: ProductionRuleId, lookahead: String) -> Self {
    Self::ShiftReduceConflict { rule, lookahead }
  }

  pub fn reduce_conflict(
    rule1: ProductionRuleId,
    rule2: ProductionRuleId,
    lookahead: String,
  ) -> Self {
    Self::ReduceConflict {
      rule1,
      rule2,
      lookahead,
    }
  }

  pub fn unresolved_states() -> Self {
    Self::UnresolvedStates
  }

  pub fn state_resolve_conflict() -> Self {
    Self::StateResolveConflict
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
      Self::UnrecognizedToken { token } => write!(f, "Unrecognized token \"{token}\""),
      Self::LabelAlreadyExists { label_id } => write!(f, "Label {label_id} already exists"),
      Self::ShiftReduceConflict { rule, lookahead } => {
        write!(
          f,
          "Shift/reduce conflict with rule {rule:?} and lookahead {lookahead}"
        )
      }
      Self::ReduceConflict {
        rule1,
        rule2,
        lookahead,
      } => {
        write!(
          f,
          "Reduce conflict between rule {rule1:?} and rule {rule2:?} with lookahead {lookahead}"
        )
      }
      Self::UnresolvedStates => write!(f, "Unresolved states while building the LR state map"),
      Self::StateResolveConflict => write!(
        f,
        "Conflict resolving states while building the LR state map"
      ),
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
