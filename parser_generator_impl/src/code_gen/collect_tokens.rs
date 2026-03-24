use crate::{code_gen::util::TokenStreamResult, ParserGeneratorResult};

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

pub trait TryCollectTokens {
  fn try_collect_tokens(self) -> TokenStreamResult;
}

impl<T, I> TryCollectTokens for T
where
  T: IntoIterator<Item = ParserGeneratorResult<I>>,
  I: Into<proc_macro2::TokenStream>,
{
  fn try_collect_tokens(self) -> TokenStreamResult {
    self
      .into_iter()
      .try_fold(proc_macro2::TokenStream::new(), |mut tokens, item| {
        tokens.extend(item?.into());
        Ok(tokens)
      })
  }
}
