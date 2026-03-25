use cknittel_util::proc_macro_util::collect_tokens::CollectTokens;
use quote::quote;

use crate::{
  annotated_grammar::parse_grammar::GrammarInfo,
  code_gen::{
    state_action_builder::state_action_function_name,
    states_enum::{enum_matcher, qualified_enum_variant_name},
    util::TokenStreamResult,
  },
};

pub fn generate_parse_loop(grammar_info: &GrammarInfo) -> TokenStreamResult {
  let lr_table = grammar_info.lr_table();

  let root_state = lr_table.root_state();
  let root_enum_state = qualified_enum_variant_name(root_state, grammar_info);

  let state_matchers = lr_table
    .states()
    .map(|state_id| {
      let enum_matcher = enum_matcher(state_id, grammar_info);
      let action_fn = state_action_function_name(state_id);
      quote! { #enum_matcher => #action_fn(&mut state), }
    })
    .collect_tokens();

  Ok(quote! {
    let mut state = ::parser_generator::parser_state::ParserState::new(
      input_stream.into_iter(),
      #root_enum_state(),
    );
    loop {
      let action = match state.state() {
        #state_matchers
      }?;

      match action {
        ::parser_generator::parser_state::ParserControl::Accept(result) => return Ok(result),
        ::parser_generator::parser_state::ParserControl::Continue => {}
      }
    }
  })
}
