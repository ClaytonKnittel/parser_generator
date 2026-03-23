use lr_table::{indexed_grammar::IndexedGrammar, lr_table::LRTable};
use quote::quote;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo, production_ref::ProductionRefName, terminal::TerminalSymbol,
  },
  code_gen::{states_enum::generate_dfa_states, util::TokenStreamResult},
};

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
