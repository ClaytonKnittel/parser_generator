use crate::{
  symbol::{SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

pub fn expect_symbol_with<F>(
  stream: &mut impl SymbolStream,
  cmp: F,
  message: impl Into<String>,
) -> ParserGeneratorResult<SymbolMeta>
where
  F: FnOnce(&SymbolT) -> bool,
{
  let sym = stream.expect_symbol()?;
  if cmp(sym.symbol_type()) {
    Ok(sym.take_meta())
  } else {
    Err(sym.meta().make_err(message))
  }
}
