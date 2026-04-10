use cknittel_util::proc_macro_util::collect_tokens::TryCollectTokens;
use quote::quote;

use crate::{
  Visibility,
  annotated_grammar::parse_grammar::GrammarInfo,
  code_gen::{
    parse_loop::generate_parse_loop,
    state_action_builder::{generate_state_action_function, root_production_type},
    states_enum::generate_dfa_states,
    util::{TokenStreamResult, unique_prefixed_ident},
  },
};

pub fn generate_parser(grammar_info: &GrammarInfo, visibility: Visibility) -> TokenStreamResult {
  let state_map = grammar_info.build_lr_state_map()?;

  let grammar_name = grammar_info.name().make_syn_ident();
  let token_type = grammar_info.terminal_type().inner_type();
  let error_type = grammar_info.error_type();

  let table_size = grammar_info.lr_table().num_states();

  let result_type = root_production_type(grammar_info);

  let dfa_states_enum = generate_dfa_states(grammar_info, &state_map)?;

  let action_functions = grammar_info
    .lr_table()
    .states()
    .map(|state_id| generate_state_action_function(state_id, grammar_info, &state_map))
    .try_collect_tokens()?;

  let parse_loop = generate_parse_loop(grammar_info)?;

  let maybe_pub = match visibility {
    Visibility::Public => quote! { pub },
    Visibility::Private => quote! {},
  };

  let input_stream = unique_prefixed_ident("input_stream");
  let iter_generic = unique_prefixed_ident("I");
  let token_generic = unique_prefixed_ident("B");
  let err_generic = unique_prefixed_ident("E");

  Ok(quote! {
    #maybe_pub struct #grammar_name;
    impl #grammar_name {
      pub const TABLE_SIZE: usize = #table_size;
    }
    impl ::parser_generator::parser::Parser for #grammar_name {
      type Token = #token_type;
      type Value = #result_type;
      type Error = #error_type;

      fn parse_fallible<#iter_generic, #token_generic, #err_generic>(
        #input_stream: #iter_generic
      ) -> ::parser_generator::error::ParserResult<Self::Value, Self::Token, Self::Error>
      where
        #iter_generic: IntoIterator<Item = ::core::result::Result<#token_generic, #err_generic>>,
        #token_generic: ::std::borrow::Borrow<#token_type>,
        #err_generic: ::parser_generator::error::ParserUserErrorOrInfallible<Self::Token, Self::Error> + Clone,
      {
        #dfa_states_enum
        #action_functions
        #parse_loop
      }
    }
  })
}
