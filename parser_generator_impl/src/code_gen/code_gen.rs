use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_state_map::{LRStateMap, LRStateType},
  lr_table::{LRTable, StateId},
};
use proc_macro::Span;
use quote::quote;

use crate::{
  annotated_grammar::parse_grammar::GrammarInfo, type_symbol::Type, ParserGeneratorError,
  ParserGeneratorResult,
};

type TokenStreamResult = ParserGeneratorResult<proc_macro2::TokenStream>;

fn generate_enum_variant(state: StateId, return_type: Option<Type>) -> proc_macro2::TokenStream {
  todo!();
}

fn generate_dfa_states<T: Clone>(
  grammar: &IndexedGrammar<T>,
  lr_table: &LRTable<T>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let grammar_name = grammar_info.name();
  let dfa_enum_name = syn::Ident::new(
    &format!("{}DfaStates", grammar_name.name()),
    grammar_name.meta().span2(),
  );

  let state_map = LRStateMap::build_from_lr_table(grammar, lr_table)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;
  let enums = lr_table
    .states()
    .map(|state| {
      (
        state,
        match state_map.state_type(state) {
          LRStateType::Reduce(production) => {
            grammar_info.lr_table_grammar();
            None
          }
          LRStateType::Terminal => Some(grammar_info.terminal_type().clone()),
          LRStateType::Root => None,
        },
      )
    })
    .map(|(state, maybe_type)| generate_enum_variant(state, maybe_type));

  Ok(quote! {
    struct #dfa_enum_name {
    }
  })
}

pub fn generate_parser<T>(
  grammar: &IndexedGrammar<T>,
  lr_table: &LRTable<T>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  Ok(quote! {})
}
