use crate::{
  fixed_map::SparseFixedSizeMap,
  grammar::ProductionNode,
  indexed_grammar::{IndexedGrammar, ProductionLabel},
  kernel::Kernel,
  position::Position,
  vocab_set::VocabSet,
  vocabulary::{AugmentedVocab, Vocabulary},
};

/// Given a closure (kernel + follow sets of production labels), computes a
/// partition over the positions of the kernel grouped by next nodes (either
/// productions or terminals). All positions at the end of their rules are
/// grouped together under `None`.
pub fn partition_closure_by_next_node<T: Vocabulary>(
  kernel: &Kernel<T>,
  follow_sets: impl IntoIterator<Item = (ProductionLabel, VocabSet<AugmentedVocab<T>>)>,
  grammar: &IndexedGrammar<T>,
) -> SparseFixedSizeMap<Option<ProductionNode<T, ProductionLabel>>, Vec<Position<T>>> {
  kernel
    .positions()
    .cloned()
    .chain(follow_sets.into_iter().flat_map(|(label, follow_set)| {
      grammar
        .production_rule_ids_for_label(label)
        .map(move |production_id| {
          Position::new_from_start_with_follow_set(production_id, follow_set.clone())
        })
    }))
    .fold(
      grammar.new_sparse_partition_closure_map(),
      |mut map, position| {
        map
          .get_mut_or_default(position.next_node(grammar).cloned())
          .push(position.clone());
        map
      },
    )
}
