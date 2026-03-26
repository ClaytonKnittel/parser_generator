use std::collections::{HashMap, HashSet};

use cknittel_util::{
  iter::JoinWith,
  proc_macro_util::collect_tokens::{CollectTokens, TryCollectTokens},
};
use lr_table::{
  grammar::ProductionRuleIndex,
  indexed_grammar::{IndexedProductionRule, ProductionLabel},
  lr_state_map::LRStateMap,
  lr_table::StateId,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{
  annotated_grammar::parse_grammar::GrammarInfo,
  code_gen::{
    constructor::build_constructor,
    states_enum::{enum_matcher, qualified_enum_variant_name},
    util::TokenStreamResult,
  },
};

/// Generates a unique name for a variable, using `node_index`.
pub fn bound_variable_ident(node_index: usize) -> syn::Ident {
  let name = format!("__v{node_index}");
  syn::Ident::new(&name, Span::call_site())
}

/// Given a set of state ids, which all must have the same associated data
/// type, generates code to pop the last value off the stack and resolve to the
/// associated data, which may have come from any of the given states.
fn extract_state(
  state_ids: impl IntoIterator<Item = StateId>,
  grammar_info: &GrammarInfo,
) -> TokenStream {
  let match_arms = state_ids
    .into_iter()
    .map(|state_id| {
      let matcher = qualified_enum_variant_name(state_id, grammar_info);
      quote! {
        #matcher(v) => v,
      }
    })
    .collect_tokens();

  quote! {
    match state.pop_state() {
      #match_arms
      _ => unsafe { ::std::hint::unreachable_unchecked() }
    }
  }
}

/// Given an index an a set of state ids, yields a statement that pops the top
/// value off the stack (which may have come from any of the state ids) and
/// bind it to a variable named `__v{node_index}`.
fn bind_production_node(
  node_index: usize,
  state_ids: impl IntoIterator<Item = StateId>,
  grammar_info: &GrammarInfo,
) -> TokenStream {
  let var_ident = bound_variable_ident(node_index);
  let extract_state = extract_state(state_ids, grammar_info);
  quote! {
    let mut #var_ident = #extract_state;
  }
}

/// Given a state id and a production rule, generates code which binds each
/// node of the state to a uniquely named variable, pulling the rules off the
/// stack.
///
/// Returns a tuple of (generated code, set of possible state ids on the top of
/// the stack after the rule reduction).
pub fn bind_production_nodes_to_locals(
  state_id: StateId,
  rule: &IndexedProductionRule,
  grammar_info: &GrammarInfo,
  state_map: &LRStateMap,
) -> (TokenStream, HashSet<StateId>) {
  let rule_len = rule.rule().len();

  let mut states = HashSet::new();
  states.insert(state_id);

  let mut tokens = TokenStream::new();

  for node_index in (0..rule_len).rev() {
    if rule.rule()[node_index].is_epsilon() {
      continue;
    }

    tokens.extend(bind_production_node(
      node_index,
      states.iter().cloned(),
      grammar_info,
    ));
    states = states
      .into_iter()
      .fold(HashSet::new(), |mut next_states, state_id| {
        next_states.extend(state_map.back_edges(state_id));
        next_states
      });
  }

  (tokens, states)
}

/// Given a list of state ids, returns an enum matcher that matches any of them.
fn match_any(
  state_ids: impl IntoIterator<Item = StateId>,
  grammar_info: &GrammarInfo,
) -> TokenStream {
  state_ids
    .into_iter()
    .map(|state_id| {
      let matcher = enum_matcher(state_id, grammar_info);
      quote! {
        #matcher
      }
    })
    .join_with(|| quote! { | })
    .collect_tokens()
}

pub fn apply_goto(
  rule_applied: ProductionRuleIndex,
  production_label: ProductionLabel,
  possible_states: HashSet<StateId>,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let mut goto_map = HashMap::<StateId, Vec<StateId>>::new();

  for state in possible_states {
    let goto = grammar_info
      .lr_table()
      .get_goto(state, production_label)
      .expect("Missing expected goto action for label");
    goto_map.entry(goto.state()).or_default().push(state);
  }

  let constructor = build_constructor(rule_applied, grammar_info)?;

  if goto_map.len() == 1 {
    let goto_state = goto_map
      .into_keys()
      .next()
      .expect("Goto map was just verified to be nonempty...");
    let next_state = qualified_enum_variant_name(goto_state, grammar_info);
    Ok(quote! {
      state.push(#next_state(#constructor));
    })
  } else {
    let match_arms = goto_map
      .into_iter()
      .map(|(goto_state, from_states)| {
        let match_from = match_any(from_states, grammar_info);
        let next_state = qualified_enum_variant_name(goto_state, grammar_info);
        Ok(quote! {
          #match_from => state.push(#next_state(__constructed)),
        })
      })
      .try_collect_tokens()?;
    Ok(quote! {
      let __constructed = #constructor;
      match state.state() {
        #match_arms
        _ => unsafe { ::std::hint::unreachable_unchecked() }
      }
    })
  }
}
