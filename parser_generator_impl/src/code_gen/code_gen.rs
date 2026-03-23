use lr_table::{
  indexed_grammar::IndexedGrammar,
  lr_state_map::{LRStateMap, LRStateType},
  lr_table::{LRTable, StateId},
};
use proc_macro::{Ident, Span};
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::collect_tokens::CollectTokens,
  type_symbol::Type,
  ParserGeneratorError, ParserGeneratorResult,
};

type TokenStreamResult = ParserGeneratorResult<proc_macro2::TokenStream>;

fn generate_enum_variant(state: StateId, return_type: Option<Type>) -> proc_macro2::TokenStream {
  let state_name = syn::Ident::new(&format!("S{}", state.id()), proc_macro2::Span::call_site());
  match return_type {
    Some(ty) => quote! { #state_name(#ty), },
    None => quote! { #state_name, },
  }
}

fn generate_dfa_states(
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
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
      let state_type = match state_map.state_type(state) {
        LRStateType::Reduce(production) => {
          let label = grammar.orig_production_label(production);
          grammar_info.label_type(label).cloned()
        }
        LRStateType::Terminal => Some(grammar_info.terminal_type().clone()),
        LRStateType::Root => None,
      };
      generate_enum_variant(state, state_type)
    })
    .collect_tokens();

  Ok(quote! {
    enum #dfa_enum_name {
      #enums
    }
  })
}

pub fn generate_parser(
  grammar: &IndexedGrammar<TerminalSymbol, ProductionRefName>,
  lr_table: &LRTable<TerminalSymbol>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let dfa_states_enum = generate_dfa_states(grammar, lr_table, grammar_info)?;

  Ok(quote! {
    #dfa_states_enum
  })
}
