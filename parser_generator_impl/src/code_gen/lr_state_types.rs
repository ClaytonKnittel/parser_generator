use std::marker::PhantomData;

use itertools::Itertools;
use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_table::{Action, LRTable, StateId},
};
use proc_macro::Span;

use crate::{type_symbol::Type, ParserGeneratorError, ParserGeneratorResult};

pub enum LRStateType {
  Reduce(Option<Type>),
  Terminal,
}

#[derive(Default)]
enum LRStateTypeBuilder {
  Reduce(Option<Type>),
  Terminal,
  #[default]
  Unknown,
}

impl LRStateTypeBuilder {
  fn finalize(self) -> ParserGeneratorResult<LRStateType> {
    match self {
      Self::Reduce(maybe_type) => Ok(LRStateType::Reduce(maybe_type)),
      Self::Terminal => Ok(LRStateType::Terminal),
      Self::Unknown => Err(ParserGeneratorError::new(
        "Internal error: LRStateType table did not resolve all states",
        Span::call_site(),
      )),
    }
  }

  fn set(&mut self, value: LRStateType) -> ParserGeneratorResult {
    Ok(match (&self, value) {
      (Self::Reduce(_), LRStateType::Reduce(_)) => {
        // TODO: compare types
      }
      (Self::Terminal, LRStateType::Terminal) => {}
      (Self::Unknown, LRStateType::Reduce(maybe_type)) => *self = Self::Reduce(maybe_type),
      (Self::Unknown, LRStateType::Terminal) => *self = Self::Terminal,
      _ => {
        return Err(ParserGeneratorError::new(
          "Internal error: conflict when building LRStateType table",
          Span::call_site(),
        ))
      }
    })
  }
}

pub struct LRStateTypes<'a> {
  types: Vec<LRStateType>,
  /// This lookup table is lifetime-bound to an instance of `LRTable`.
  _phantom: PhantomData<&'a ()>,
}

impl<'a> LRStateTypes<'a> {
  pub fn build_from_lr_table<T: Clone>(
    grammar: &IndexedGrammar<T>,
    lr_table: &'a LRTable<T>,
  ) -> ParserGeneratorResult<Self> {
    let mut types = (0..lr_table.num_states())
      .map(|_| LRStateTypeBuilder::default())
      .collect_vec();

    for state in lr_table.states() {
      for (_, action) in lr_table.state_actions(state, grammar) {
        if let Action::Shift { next_state } = action {
          types[next_state.id()].set(LRStateType::Terminal)?;
        }
      }
    }

    let types = types
      .into_iter()
      .map(LRStateTypeBuilder::finalize)
      .collect::<Result<_, _>>()?;

    Ok(Self {
      types,
      _phantom: PhantomData,
    })
  }

  pub fn state_type(&self, state: StateId) -> &LRStateType {
    &self.types[state.id()]
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use lr_table::{
    grammar::Grammar,
    indexed_grammar::IndexedGrammar,
    lr_table::{LRTable, StateId},
  };

  use crate::code_gen::lr_state_types::LRStateTypes;

  #[gtest]
  fn test_single_rule() {
    let grammar = Grammar::from_grammar_str("A -> a").unwrap();

    let grammar = IndexedGrammar::build(&grammar).unwrap();
    let lr_table = LRTable::build(&grammar).unwrap();
    // let label_a = *label_map.get("A").unwrap();
    // let a_id = grammar.vocab().token_to_id(&b'a').unwrap();
    let state_table = LRStateTypes::build_from_lr_table(&grammar, &lr_table).unwrap();

    expect_eq!(state_table.state_type(StateId::from_id(0)), 0);
  }
}
