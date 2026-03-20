use crate::{
  error::InterceptResult,
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

#[derive(Clone)]
pub struct Type {
  pub meta: SymbolMeta,
}

impl Type {
  pub fn as_type(&self) -> syn::Type {
    syn::Type::Verbatim(self.meta.tokens().clone().into())
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    // Consume everything up to the next `=>` or `;`. The two contexts that
    // types appear in - terminal type declaration and production rule return
    // type - are always proceeded by one of these two symbols.

    let first_sym = stream.expect_symbol()?;
    let mut meta = first_sym.meta().clone();

    loop {
      let sym = stream.peek_expect_symbol().intercept("expected type")?;

      // Break when we first encounter '=>' / ';'
      if matches!(
        sym.symbol_type(),
        SymbolT::Op(Operator::Arrow | Operator::Semicolon)
      ) {
        break;
      }

      meta.merge(sym.take().meta())?;
    }

    Ok(Self { meta })
  }
}
