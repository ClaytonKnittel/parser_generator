use quote::ToTokens;

use crate::{
  ParserGeneratorResult,
  error::InterceptResult,
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
};

#[derive(Clone)]
pub struct Type {
  pub meta: SymbolMeta,
}

impl Type {
  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    // Consume everything up to the next `=>` or `;`. The two contexts that
    // types appear in - terminal type declaration and production rule return
    // type - are always proceeded by one of these two symbols.

    let first_sym = stream.expect_symbol()?;
    let mut meta = first_sym.take_meta();

    loop {
      let sym = stream.peek_expect_symbol().intercept("expected type")?;

      // Break when we first encounter '=>' / ';'
      if matches!(
        sym.symbol_type(),
        SymbolT::Op(Operator::Arrow | Operator::Semicolon)
      ) {
        break;
      }

      meta.merge(sym.take().meta()).intercept("callsite 3")?;
    }

    Ok(Self { meta })
  }

  pub fn cmp_tokens(&self, other: &Self) -> bool {
    self.meta.tokens().to_string() == other.meta.tokens().to_string()
  }
}

impl ToTokens for Type {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.extend(self.meta.tokens().clone());
  }
}
