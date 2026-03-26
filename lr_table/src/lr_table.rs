use std::{
  fmt::{Debug, Display},
  marker::PhantomData,
};

use itertools::{IntoChunks, Itertools};

use crate::{
  bit_set::BitSet,
  closure::partition_closure_by_next_node,
  error::{LRTableError, LRTableResult},
  first_map::FirstTable,
  fixed_map::{Label, SparseFixedSizeMap},
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionLabel, ProductionRuleId, SparsePartitionMap},
  kernel::Kernel,
  kernel_table::KernelTable,
  position::Position,
  vocabulary::{AugmentedTokenId, AugmentedVocabToken},
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
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

struct LRTableEntryBuilder {
  /// A map from token -> action for all actions that may be taken from this
  /// state.
  actions: SparseFixedSizeMap<AugmentedTokenId, Action>,
  /// A map from production label -> goto action for all production labels
  /// which may return to this state after reducing.
  gotos: SparseFixedSizeMap<ProductionLabel, GotoAction>,
}

impl LRTableEntryBuilder {
  fn new<T, L>(grammar: &IndexedGrammar<T, L>) -> Self {
    Self {
      actions: grammar.new_sparse_augmented_vocab_map(),
      gotos: grammar.new_sparse_production_label_map(),
    }
  }

  /// Builds the LRTableEntry for the state given the partitions of a kernel +
  /// closure, which is the set of actions to be taken from this state.
  fn try_build_from_partitions<T: Clone + ToString, L>(
    partitions: SparsePartitionMap<Vec<Position>>,
    kernel_table: &mut KernelTable,
    grammar: &IndexedGrammar<T, L>,
  ) -> LRTableResult<LRTableEntryBuilder> {
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
                .all(|token| matches!(token, AugmentedVocabToken::EndOfStream))
            );
            builder.add_accept(AugmentedVocabToken::EndOfStream)?;
          } else {
            for follow_token in position.follow_set().iter() {
              builder.add_reduce_action(follow_token, position.rule(), grammar)?;
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

  fn add_shift_action(&mut self, token: AugmentedTokenId, next_state: StateId) -> LRTableResult {
    self
      .actions
      .try_insert(&token, Action::Shift { next_state })
  }

  fn add_reduce_action<T: Clone + ToString, L>(
    &mut self,
    token: AugmentedTokenId,
    rule: ProductionRuleId,
    grammar: &IndexedGrammar<T, L>,
  ) -> LRTableResult {
    self
      .actions
      .try_insert(&token, Action::Reduce { rule })
      .map_err(|_| {
        let existing_action = self.actions.get(&token).unwrap();
        let lookahead = grammar.vocab().id_to_token(token).to_string();
        match existing_action {
          Action::Reduce { rule: other_rule } => {
            LRTableError::reduce_conflict(rule, *other_rule, lookahead)
          }
          Action::Shift { .. } => LRTableError::shift_reduce_conflict(rule, lookahead),
          Action::Accept => {
            LRTableError::reduce_conflict(rule, grammar.root_production_rule(), lookahead)
          }
        }
      })
  }

  fn add_goto_action(&mut self, label: ProductionLabel, next_state: StateId) -> LRTableResult {
    self.gotos.try_insert(&label, GotoAction(next_state))
  }

  fn add_accept(&mut self, token: AugmentedTokenId) -> LRTableResult {
    self.actions.try_insert(&token, Action::Accept)
  }

  /// Flattens the entry builder into action/goto vecs which correspond to rows
  /// in the `LRTable` for this state.
  fn into_vecs<T: Clone, L>(
    self,
    grammar: &IndexedGrammar<T, L>,
  ) -> (
    impl Iterator<Item = Option<Action>>,
    impl Iterator<Item = Option<GotoAction>>,
  ) {
    (
      grammar
        .vocab()
        .for_each_id()
        .map(move |token| self.actions.get(&token).cloned()),
      grammar
        .all_production_labels()
        .map(move |label| self.gotos.get(&label).cloned()),
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

impl<T> LRTable<T> {
  fn generate_actions<L>(
    grammar: &IndexedGrammar<T, L>,
  ) -> impl Iterator<Item = LRTableResult<LRTableEntryBuilder>>
  where
    T: Clone + ToString,
  {
    let first_set = FirstTable::build_from_grammar(grammar);
    let mut kernel_table = KernelTable::new();

    // Construct the root kernel, which exists of only the root rule.
    let root_label = grammar.root_production_label();
    let initial_kernel = Kernel::new(
      grammar
        .production_rule_ids_for_label(root_label)
        .map(|rule_id| Position::new_top_level(rule_id, grammar.vocab()))
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

  pub fn build<L>(grammar: &IndexedGrammar<T, L>) -> LRTableResult<Self>
  where
    T: Clone + ToString,
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

  pub fn root_state(&self) -> StateId {
    StateId(0)
  }

  pub fn get_action(&self, state: StateId, token: AugmentedTokenId) -> Option<Action> {
    let index = self.vocab_size() * state.id() + token.id();
    self.action_table[index]
  }

  pub fn get_goto(&self, state: StateId, production_label: ProductionLabel) -> Option<GotoAction> {
    let index = self.num_production_labels() * state.id() + production_label.id();
    self.goto_table[index]
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

  fn actions_iter(&self) -> IntoChunks<impl Iterator<Item = &Option<Action>>> {
    self.action_table.iter().chunks(self.vocab_size())
  }

  fn gotos_iter(&self) -> IntoChunks<impl Iterator<Item = &Option<GotoAction>>> {
    self.goto_table.iter().chunks(self.num_production_labels())
  }

  pub fn num_states(&self) -> usize {
    self.num_states
  }

  pub fn states(&self) -> impl Iterator<Item = StateId> {
    (0..self.num_states).map(StateId)
  }

  /// Returns an iterator over all actions for a given state. The iterator
  /// yields pairs (token, action), where consuming the given token in this
  /// state should trigger the corresponding action.
  pub fn state_actions<L>(
    &self,
    state: StateId,
    grammar: &IndexedGrammar<T, L>,
  ) -> impl Iterator<Item = (AugmentedVocabToken<T>, &Action)>
  where
    T: Clone,
  {
    debug_assert_eq!(grammar.vocab().size(), self.vocab_size());
    let vocab_size = grammar.vocab().size();
    let state_offset = state.0 * vocab_size;
    self.action_table[state_offset..state_offset + vocab_size]
      .iter()
      .zip(grammar.vocab().for_each_id())
      .filter_map(|(action, token_id)| {
        action
          .as_ref()
          .map(|action| (grammar.vocab().id_to_token(token_id), action))
      })
  }

  /// Returns an iterator over all GOTO actions for a given state. The iterator
  /// yields pairs (production rule, action). When that particular production
  /// reduces into this state, this GOTO action should be applied.
  pub fn goto_actions<L>(
    &self,
    state: StateId,
    grammar: &IndexedGrammar<T, L>,
  ) -> impl Iterator<Item = (ProductionLabel, &GotoAction)>
  where
    T: Clone,
  {
    debug_assert_eq!(self.num_production_labels(), grammar.labels_count());
    let num_production_labels = grammar.labels_count();
    let state_offset = state.0 * num_production_labels;
    self.goto_table[state_offset..state_offset + num_production_labels]
      .iter()
      .zip(grammar.all_production_labels())
      .filter_map(|(action, production_label)| {
        action.as_ref().map(|action| (production_label, action))
      })
  }
}

impl<T: Label + Display> Display for LRTable<T> {
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

    // Print header
    write!(f, "{:count$}  ", "", count = state_index_print_width)?;
    for token in relevant_vocab.for_each().map(AugmentedTokenId::from_id) {
      write!(
        f,
        "{:count$} ",
        format!("{token}"),
        count = action_print_width
      )?;
    }
    writeln!(f)?;

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
