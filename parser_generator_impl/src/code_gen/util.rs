use proc_macro2::Span;

use crate::ParserGeneratorResult;

pub type TokenStreamResult = ParserGeneratorResult<proc_macro2::TokenStream>;

/// Adds a prefix to symbols that will be visible from user code (e.g.
/// production constructors) to prevent unexpected/confusing naming conflicts.
pub fn unique_prefixed_ident(name: &str) -> proc_macro2::Ident {
  proc_macro2::Ident::new(&format!("__parser_generator_{name}"), Span::call_site())
}
