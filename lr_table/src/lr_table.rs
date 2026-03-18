use std::{fmt::Debug, hash::Hash};

use itertools::Itertools;

use crate::{
  closure::closure_follow_sets,
  error::LRTableResult,
  first_map::FirstTable,
  fixed_map::{Label, SparseFixedSizeMap},
  grammar::{Grammar, ProductionNode},
  indexed_grammar::{IndexedGrammar, ProductionLabel, ProductionRuleId},
  kernel::Kernel,
  kernel_table::KernelTable,
  partition_closure::partition_closure_by_next_node,
  position::Position,
  vocabulary::{AugmentedVocab, Vocabulary},
};

#[derive(Clone, Copy)]
pub struct StateId(usize);

impl StateId {
  pub fn new(id: usize) -> Self {
    Self(id)
  }

  pub fn id(&self) -> usize {
    self.0
  }
}

#[derive(Clone, Copy)]
enum Action {
  Shift { next_state: StateId },
  Reduce { rule: ProductionRuleId },
  Accept,
}

impl Debug for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Shift { next_state } => write!(f, "s{}", next_state.0),
      Self::Reduce { rule } => write!(f, "s{}", rule.id()),
      Self::Accept => write!(f, "acc"),
    }
  }
}

#[derive(Clone, Copy)]
struct GotoAction(StateId);

impl Debug for GotoAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "GO({})", self.0.id())
  }
}

struct LRTableEntryBuilder<T> {
  /// A map from token -> action for all actions that may be taken from this
  /// state.
  actions: SparseFixedSizeMap<AugmentedVocab<T>, Action>,
  /// A map from production label -> goto action for all production labels
  /// which may return to this state after reducing.
  gotos: SparseFixedSizeMap<ProductionLabel, GotoAction>,
}

impl<T: Vocabulary> LRTableEntryBuilder<T> {
  fn new(grammar: &IndexedGrammar<T>) -> Self {
    Self {
      actions: grammar.new_sparse_augmented_vocab_map(),
      gotos: grammar.new_sparse_production_label_map(),
    }
  }

  /// Builds the LRTableEntry for the state given the partitions of a kernel +
  /// closure, which is the set of actions to be taken from this state.
  fn try_build_from_partitions(
    partitions: SparseFixedSizeMap<Option<ProductionNode<T, ProductionLabel>>, Vec<Position<T>>>,
    kernel_table: &mut KernelTable<T>,
    grammar: &IndexedGrammar<T>,
  ) -> LRTableResult<LRTableEntryBuilder<T>> {
    let mut builder = LRTableEntryBuilder::new(grammar);

    for (maybe_node, mut positions) in partitions {
      let Some(node) = maybe_node else {
        // This is the partition of positions at the end of their rules. Go
        // through each individual position in the partition and add a reduce
        // action for each token in the rule's follow set.
        for position in positions {
          if position.rule() == grammar.root_production_rule() {
            // Special case for the root rule, which is the only rule that can
            // accept.
            debug_assert!(
              position
                .follow_set()
                .iter()
                .all(|token| matches!(token, AugmentedVocab::EndOfStream))
            );
            builder.add_accept(AugmentedVocab::EndOfStream)?;
          } else {
            for follow_token in position.follow_set().iter() {
              builder.add_reduce_action(follow_token, position.rule())?;
            }
          }
        }
        continue;
      };
      // All positions in this partition should have been grouped according to
      // their next nodes.
      debug_assert!(
        positions
          .iter()
          .all(|position| position.next_node(grammar) == Some(&node))
      );

      // Advance all positions in the partition, given that these positions are
      // not at the end of their rules, and build a kernel out of them.
      Position::advance_all(positions.iter_mut(), grammar);
      let kernel = Kernel::new(positions);
      let id = kernel_table.get_or_insert(kernel);

      match node {
        ProductionNode::Production(label) => {
          builder.add_goto_action(label, id)?;
        }
        ProductionNode::Terminal(terminal) => {
          builder.add_shift_action(terminal, id)?;
        }
      }
    }

    Ok(builder)
  }

  fn add_shift_action(&mut self, token: AugmentedVocab<T>, next_state: StateId) -> LRTableResult {
    self.actions.try_insert(token, Action::Shift { next_state })
  }

  fn add_reduce_action(
    &mut self,
    token: AugmentedVocab<T>,
    rule: ProductionRuleId,
  ) -> LRTableResult {
    self.actions.try_insert(token, Action::Reduce { rule })
  }

  fn add_goto_action(&mut self, label: ProductionLabel, next_state: StateId) -> LRTableResult {
    self.gotos.try_insert(label, GotoAction(next_state))
  }

  fn add_accept(&mut self, token: AugmentedVocab<T>) -> LRTableResult {
    self.actions.try_insert(token, Action::Accept)
  }

  /// Flattens the entry builder into action/goto vecs which correspond to rows
  /// in the `LRTable` for this state.
  fn into_vecs(
    self,
    grammar: &IndexedGrammar<T>,
  ) -> (
    impl Iterator<Item = Option<Action>>,
    impl Iterator<Item = Option<GotoAction>>,
  ) {
    (
      AugmentedVocab::<T>::for_each().map(move |token| self.actions.get(token).cloned()),
      grammar
        .all_production_labels()
        .map(move |label| self.gotos.get(label).cloned()),
    )
  }
}

pub struct LRTable {
  /// A vocab_size * num_states sized table of actions.
  action_table: Vec<Option<Action>>,
  /// A num_production_labels * num_states sized table of goto actions.
  goto_table: Vec<Option<GotoAction>>,
  num_states: usize,
}

impl LRTable {
  fn generate_actions<T: Vocabulary>(
    grammar: &IndexedGrammar<T>,
  ) -> impl Iterator<Item = LRTableResult<LRTableEntryBuilder<T>>> {
    let first_set = FirstTable::build_from_grammar(grammar);
    let mut kernel_table = KernelTable::<T>::new();

    // Construct the root kernel, which exists of only the root rule.
    let root_label = grammar.root_production_label();
    let initial_kernel = Kernel::new(
      grammar
        .production_rule_ids_for_label(root_label)
        .map(|rule_id| Position::new_top_level(rule_id))
        .collect(),
    );
    let id = kernel_table.get_or_insert(initial_kernel);
    debug_assert_eq!(id.0, 0);

    (0..).map_while(move |state_id| {
      let state_id = StateId::new(state_id);
      // Retrieve the next state from the table. The table is filled up as
      // rules are explored. If we don't find an entry after reaching a
      // particular `StateId`, then we've already explored all reachable
      // kernels.
      let kernel = kernel_table.get_state(state_id)?;

      // Compute the closure of the kernel.
      let follow_sets = closure_follow_sets(kernel, grammar, &first_set);

      // Partition the positions of the closure by next tokens.
      let partitions = partition_closure_by_next_node(kernel, follow_sets, grammar);

      Some(LRTableEntryBuilder::try_build_from_partitions(
        partitions,
        &mut kernel_table,
        grammar,
      ))
    })
  }

  pub fn build<T: Clone + Vocabulary, L: Clone + Eq + Hash>(
    grammar: &Grammar<T, L>,
  ) -> LRTableResult<Self> {
    let indexed_grammar = IndexedGrammar::build(grammar);
    Self::generate_actions(&indexed_grammar)
      .map(|entry_builder| {
        entry_builder.map(|entry_builder| entry_builder.into_vecs(&indexed_grammar))
      })
      .try_fold(
        Self {
          action_table: Vec::new(),
          goto_table: Vec::new(),
          num_states: 0,
        },
        |mut lr_table, actions| {
          actions.map(|(actions, goto_actions)| {
            lr_table.action_table.extend(actions);
            lr_table.goto_table.extend(goto_actions);
            lr_table.num_states += 1;
            lr_table
          })
        },
      )
  }

  fn vocab_size(&self) -> usize {
    let num_actions = self.action_table.len();
    debug_assert!(num_actions.is_multiple_of(self.num_states));
    num_actions / self.num_states
  }

  fn num_production_labels(&self) -> usize {
    let num_gotos = self.goto_table.len();
    debug_assert!(num_gotos.is_multiple_of(self.num_states));
    num_gotos / self.num_states
  }
}

impl Debug for LRTable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let vocab_size = self.vocab_size();
    let num_production_labels = self.num_production_labels();

    let action_chunks = self.action_table.iter().chunks(vocab_size);
    let mut actions_iter = action_chunks.into_iter();
    let goto_chunks = self.goto_table.iter().chunks(num_production_labels);
    let mut gotos_iter = goto_chunks.into_iter();
    for state in 0..self.num_states {
      let action_set = actions_iter.next().unwrap();
      let goto_set = gotos_iter.next().unwrap();

      write!(
        f,
        "{state}: {} : {:?}",
        action_set
          .map(|action| match action {
            Some(action) => format!("{action:?}"),
            None => "_".to_owned(),
          })
          .join(" "),
        goto_set
          .map(|action| match action {
            Some(action) => format!("{action:?}"),
            None => "_".to_owned(),
          })
          .join(" ")
      )?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{grammar::Grammar, lr_table::LRTable};

  #[gtest]
  fn test() {
    let grammar = Grammar::from_grammar_str(r#"A -> a"#).unwrap();
    let x = LRTable::build(&grammar).unwrap();

    println!("{x:?}");

    expect_true!(false);

    // let (indexed, label_map) = IndexedGrammar::build_with_label_map(&grammar);
    // let label_a = *label_map.get("A").unwrap();
    // let production_id_a = indexed
    //   .production_rule_ids_for_label(label_a)
    //   .next()
    //   .unwrap();
  }
}
