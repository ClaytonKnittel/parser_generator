use crate::{
  symbol::{SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

pub struct Ident {
  name: String,
  meta: SymbolMeta,
}

impl Ident {
  pub fn make_syn_ident(&self) -> syn::Ident {
    syn::Ident::new(&self.name, *self.meta.span())
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let sym = stream.expect_symbol()?;
    match sym.symbol_type() {
      SymbolT::Ident(name) => Ok(Self {
        name: name.clone(),
        meta: sym.take_meta(),
      }),
      _ => Err(sym.meta().make_err("Expected identifier")),
    }
  }
}
