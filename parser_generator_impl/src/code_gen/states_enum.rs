use cknittel_util::proc_macro_util::collect_tokens::CollectTokens;
use lr_table::{
  lr_state_map::{LRStateMap, LRStateType},
  lr_table::StateId,
};
use quote::quote;

use crate::{
  annotated_grammar::{parse_grammar::GrammarInfo, terminal::UserDefinedSymbol},
  code_gen::util::TokenStreamResult,
  type_symbol::Type,
};

pub fn enum_name(grammar_info: &GrammarInfo) -> syn::Ident {
  let grammar_name = grammar_info.name();
  syn::Ident::new(
    &format!("{}DfaStates", grammar_name.name()),
    *grammar_name.meta().span(),
  )
}

pub fn enum_variant_name(state: StateId) -> syn::Ident {
  syn::Ident::new(&format!("S{}", state.id()), proc_macro2::Span::call_site())
}

pub fn qualified_enum_variant_name(
  state: StateId,
  grammar_info: &GrammarInfo,
) -> proc_macro2::TokenStream {
  let enum_name = enum_name(grammar_info);
  let variant = enum_variant_name(state);
  quote! { #enum_name::#variant }
}

fn state_type(
  state: StateId,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap<UserDefinedSymbol>,
) -> Option<Type> {
  match state_map.state_type(state) {
    LRStateType::Reduce(production) => {
      let label = grammar_info.grammar().orig_production_label(*production);
      grammar_info.label_type(label).cloned()
    }
    LRStateType::Terminal(terminal) => {
      //
      Some(grammar_info.terminal_type().inner_type().clone())
    }
    LRStateType::Root => None,
  }
}

fn generate_enum_variant(state: StateId, return_type: Option<Type>) -> proc_macro2::TokenStream {
  let state_name = enum_variant_name(state);
  let return_type = match return_type {
    Some(ty) => quote! { #ty },
    None => quote! { () },
  };
  quote! { #state_name(#return_type), }
}

pub fn enum_matcher(state: StateId, grammar_info: &GrammarInfo) -> proc_macro2::TokenStream {
  let name = qualified_enum_variant_name(state, grammar_info);
  quote! { #name(..) }
}

pub fn generate_dfa_states(
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap<UserDefinedSymbol>,
) -> TokenStreamResult {
  let dfa_enum_name = enum_name(grammar_info);

  let enums = grammar_info
    .lr_table()
    .states()
    .map(|state| generate_enum_variant(state, state_type(state, grammar_info, state_map)))
    .collect_tokens();

  Ok(quote! {
    enum #dfa_enum_name {
      #enums
    }
  })
}
