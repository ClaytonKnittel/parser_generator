use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_state_map::{LRStateMap, LRStateType},
  lr_table::{LRTable, StateId},
};
use proc_macro::Span;
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::{collect_tokens::CollectTokens, util::TokenStreamResult},
  type_symbol::Type,
  ParserGeneratorError,
};

fn enum_name(grammar_info: &GrammarInfo) -> syn::Ident {
  let grammar_name = grammar_info.name();
  syn::Ident::new(
    &format!("{}DfaStates", grammar_name.name()),
    grammar_name.meta().span2(),
  )
}

fn state_type(
  state: StateId,
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap,
) -> Option<Type> {
  match state_map.state_type(state) {
    LRStateType::Reduce(production) => {
      let label = grammar.orig_production_label(production);
      grammar_info.label_type(label).cloned()
    }
    LRStateType::Terminal => Some(grammar_info.terminal_type().clone()),
    LRStateType::Root => None,
  }
}

fn generate_enum_variant(state: StateId, return_type: Option<Type>) -> proc_macro2::TokenStream {
  let state_name = syn::Ident::new(&format!("S{}", state.id()), proc_macro2::Span::call_site());
  match return_type {
    Some(ty) => quote! { #state_name(#ty), },
    None => quote! { #state_name, },
  }
}

pub fn generate_dfa_states(
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let dfa_enum_name = enum_name(grammar_info);

  let state_map = LRStateMap::build_from_lr_table(grammar, lr_table)
    .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))?;
  let enums = lr_table
    .states()
    .map(|state| generate_enum_variant(state, state_type(state, grammar, grammar_info, &state_map)))
    .collect_tokens();

  Ok(quote! {
    enum #dfa_enum_name {
      #enums
    }
  })
}
