use quote::quote;

use crate::code_gen::util::TokenStreamResult;

pub fn generate_parse_loop() -> TokenStreamResult {
  Ok(quote! {
    let mut states = vec![];
    loop {
    }
  })
}
