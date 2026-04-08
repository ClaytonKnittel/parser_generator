use proc_macro_error::proc_macro_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

fn build_user_error_impl(input: DeriveInput) -> TokenStream {
  let ident = input.ident;
  quote! {
    impl From<::std::convert::Infallible> for #ident {
      fn from(value: ::std::convert::Infallible) -> Self {
        match value {}
      }
    }
    impl ::parser_generator::error::ParserUserError for #ident {}
  }
}

#[proc_macro_error]
#[proc_macro_derive(ParserUserError)]
pub fn derive_parser_user_error(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  build_user_error_impl(input).into()
}
