use std::collections::{hash_map::Entry, HashMap};

use crate::{
  annotated_grammar::production_ref::ProductionRefName, type_symbol::Type, ParserGeneratorResult,
};

fn types_equal(t1: Option<&Type>, t2: Option<&Type>) -> bool {
  match (t1, t2) {
    (Some(t1), Some(t2)) => t1.cmp_tokens(t2),
    (None, None) => true,
    _ => false,
  }
}

#[derive(Default)]
pub struct LabelTypeMap {
  label_types: HashMap<ProductionRefName, Option<Type>>,
}

impl LabelTypeMap {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn get(&self, label: &ProductionRefName) -> Option<&Type> {
    self.label_types[label].as_ref()
  }

  pub fn add(
    &mut self,
    label: ProductionRefName,
    label_type: Option<Type>,
  ) -> ParserGeneratorResult {
    match self.label_types.entry(label) {
      Entry::Occupied(entry) => {
        let existing = entry.get().as_ref();
        let label_type = label_type.as_ref();
        if !types_equal(existing, label_type) {
          return Err(
            label_type
              .or(existing)
              .unwrap()
              .meta
              .make_err("Production rules for the same label have different return types"),
          );
        }
      }
      Entry::Vacant(entry) => {
        entry.insert(label_type);
      }
    }

    Ok(())
  }
}
