use proc_macro2::Span;

use crate::ParserGeneratorResult;

pub type TokenStreamResult = ParserGeneratorResult<proc_macro2::TokenStream>;

/// A prefix to prepend to symbols that will be visible from user code (e.g.
/// production constructors) to prevent unexpected/confusing naming conflicts.
pub fn unique_prefix() -> &'static str {
  "__parser_generator_"
}

pub fn add_unique_prefix(name: &str) -> String {
  format!("{}{name}", unique_prefix())
}

pub fn unique_prefixed_ident(name: &str) -> proc_macro2::Ident {
  proc_macro2::Ident::new(&add_unique_prefix(name), Span::call_site())
}
