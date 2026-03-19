use std::{
  fmt::{Debug, Display},
  marker::PhantomData,
};

use itertools::{IntoChunks, Itertools};

use crate::{
  bit_set::BitSet,
  closure::partition_closure_by_next_node,
  error::LRTableResult,
  first_map::FirstTable,
  fixed_map::{Label, SparseFixedSizeMap},
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionLabel, ProductionRuleId},
  kernel::Kernel,
  kernel_table::KernelTable,
  position::Position,
  vocabulary::{AugmentedVocab, Vocabulary},
};

#[derive(Clone, Copy, Default)]
pub struct StateId(usize);

impl StateId {
  pub fn new(id: usize) -> Self {
    Self(id)
  }

  pub fn id(&self) -> usize {
    self.0
  }
}

impl Debug for StateId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "s({})", self.0)
  }
}

#[derive(Clone, Copy)]
pub enum Action {
  Shift { next_state: StateId },
  Reduce { rule: ProductionRuleId },
  Accept,
}

impl Display for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Shift { next_state } => write!(f, "s{}", next_state.0),
      Self::Reduce { rule } => write!(f, "r{}", rule.id()),
      Self::Accept => write!(f, "acc"),
    }
  }
}

impl Debug for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

#[derive(Clone, Copy)]
pub struct GotoAction(StateId);

impl GotoAction {
  pub fn state(&self) -> StateId {
    self.0
  }
}

impl Display for GotoAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "GO({})", self.0.id())
  }
}

impl Debug for GotoAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
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
          debug_assert!(position.at_end_of_rule(grammar));

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
          .all(|position| !position.at_end_of_rule(grammar))
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

pub struct LRTable<T> {
  /// A vocab_size * num_states sized table of actions.
  action_table: Vec<Option<Action>>,
  /// A num_production_labels * num_states sized table of goto actions.
  goto_table: Vec<Option<GotoAction>>,
  num_states: usize,
  _phantom: PhantomData<T>,
}

impl<T: Vocabulary> LRTable<T> {
  fn generate_actions(
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

      // Partition the positions of the kernel's closure by next tokens.
      let partitions = partition_closure_by_next_node(kernel, grammar, &first_set);

      Some(LRTableEntryBuilder::try_build_from_partitions(
        partitions,
        &mut kernel_table,
        grammar,
      ))
    })
  }

  pub fn build(grammar: &IndexedGrammar<T>) -> LRTableResult<Self>
  where
    T: Clone,
  {
    Self::generate_actions(grammar)
      .map(|entry_builder| entry_builder.map(|entry_builder| entry_builder.into_vecs(grammar)))
      .try_fold(
        Self {
          action_table: Vec::new(),
          goto_table: Vec::new(),
          num_states: 0,
          _phantom: PhantomData,
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

  pub fn get_action(&self, state: StateId, token: AugmentedVocab<T>) -> Option<Action> {
    let index = self.vocab_size() * state.id() + token.ordinal();
    self.action_table[index]
  }

  pub fn get_goto(&self, state: StateId, production_label: ProductionLabel) -> Option<GotoAction> {
    let index = self.num_production_labels() * state.id() + production_label.id();
    self.goto_table[index]
  }
}

impl<T> LRTable<T> {
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

  fn actions_iter(&self) -> IntoChunks<impl Iterator<Item = &Option<Action>>> {
    self.action_table.iter().chunks(self.vocab_size())
  }

  fn gotos_iter(&self) -> IntoChunks<impl Iterator<Item = &Option<GotoAction>>> {
    self.goto_table.iter().chunks(self.num_production_labels())
  }
}

impl<T> Display for LRTable<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut action_print_width = 1;
    let relevant_vocab =
      self
        .actions_iter()
        .into_iter()
        .fold(BitSet::new(self.vocab_size()), |mut vocab, actions| {
          for (i, action) in actions.enumerate() {
            if let Some(action) = action {
              vocab.set(i);
              action_print_width = action_print_width.max(format!("{action}").len());
            }
          }
          vocab
        });
    let goto_print_width = self
      .gotos_iter()
      .into_iter()
      .map(|gotos| {
        gotos
          .map(|goto| match goto {
            Some(goto) => format!("{goto}").len(),
            None => 0,
          })
          .max()
          .unwrap_or_default()
      })
      .max()
      .unwrap_or(1);

    let state_index_print_width = format!("{}", self.num_states).len();

    let action_chunks = self.actions_iter();
    let mut actions_iter = action_chunks.into_iter();
    let goto_chunks = self.gotos_iter();
    let mut gotos_iter = goto_chunks.into_iter();
    for state in 0..self.num_states {
      let action_set = actions_iter.next().unwrap().collect_vec();
      let goto_set = gotos_iter.next().unwrap();

      write!(
        f,
        "{state:>width$}: {} : {}",
        relevant_vocab
          .for_each()
          .map(|i| match action_set[i] {
            Some(action) => format!(
              "{:width$}",
              format!("{}", action),
              width = action_print_width
            ),
            None => format!("{:width$}", "_", width = action_print_width),
          })
          .join(" "),
        goto_set
          .map(|goto| match goto {
            Some(goto) => format!("{:width$}", format!("{goto}"), width = goto_print_width),
            None => format!("{:width$}", "_", width = goto_print_width),
          })
          .join(" "),
        width = state_index_print_width
      )?;

      if state < self.num_states - 1 {
        writeln!(f)?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{grammar::Grammar, indexed_grammar::IndexedGrammar, lr_table::LRTable};

  #[gtest]
  fn test() {
    let grammar = Grammar::from_grammar_str(
      r#"T -> S
         S -> S p P
         S -> P
         P -> P x V
         P -> V
         V -> a
         V -> b
         V -> c"#,
    )
    .unwrap();
    let indexed_grammar = IndexedGrammar::build(&grammar);
    let x = LRTable::build(&indexed_grammar).unwrap();

    println!("{x}");

    expect_true!(false);
  }
}
