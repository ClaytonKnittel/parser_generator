pub trait CollectTokens {
  fn collect_tokens(self) -> proc_macro2::TokenStream;
}

impl<T, I> CollectTokens for T
where
  T: IntoIterator<Item = I>,
  I: Into<proc_macro2::TokenStream>,
{
  fn collect_tokens(self) -> proc_macro2::TokenStream {
    self
      .into_iter()
      .fold(proc_macro2::TokenStream::new(), |mut tokens, item| {
        tokens.extend(item.into());
        tokens
      })
  }
}
