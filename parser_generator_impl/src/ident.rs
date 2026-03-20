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
  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let sym = stream.expect_symbol()?;
    match sym.symbol_type() {
      SymbolT::Ident(name) => Ok(Self {
        name: name.clone(),
        meta: sym.meta().clone(),
      }),
      _ => Err(sym.meta().make_err("Expected identifier")),
    }
  }
}
