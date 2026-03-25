use cknittel_util::proc_macro_util::collect_tokens::TryCollectTokens;
use quote::quote;

use crate::{
  annotated_grammar::parse_grammar::GrammarInfo,
  code_gen::{
    parse_loop::generate_parse_loop,
    state_action_builder::{generate_state_action_function, root_production_type},
    states_enum::generate_dfa_states,
    util::TokenStreamResult,
  },
};

pub fn generate_parser(grammar_info: &GrammarInfo) -> TokenStreamResult {
  let state_map = grammar_info.build_lr_state_map()?;

  let grammar_name = grammar_info.name().make_syn_ident();
  let token_type = grammar_info.terminal_type();

  let result_type = root_production_type(grammar_info);

  let dfa_states_enum = generate_dfa_states(grammar_info, &state_map)?;

  let action_functions = grammar_info
    .lr_table()
    .states()
    .map(|state_id| generate_state_action_function(state_id, grammar_info, &state_map))
    .try_collect_tokens()?;

  let parse_loop = generate_parse_loop(grammar_info)?;

  Ok(quote! {
    struct #grammar_name;
    impl ::parser_generator::parser::Parser for #grammar_name {
      type Token = #token_type;
      type Value = #result_type;

      fn parse<I, B>(
        input_stream: I
      ) -> ::parser_generator::error::ParserResult<Self::Value>
      where
        I: IntoIterator<Item = B>,
        B: ::std::borrow::Borrow<#token_type>,
      {
        #dfa_states_enum
        #action_functions
        #parse_loop
      }
    }
  })
}
