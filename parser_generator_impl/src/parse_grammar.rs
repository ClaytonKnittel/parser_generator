use crate::{
  error::InterceptResult,
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  Grammar, ParserGeneratorResult,
};

#[derive(Clone)]
pub struct Type {
  pub meta: SymbolMeta,
}

impl Type {
  pub fn as_type(&self) -> syn::Type {
    syn::Type::Verbatim(self.meta.tokens().clone().into())
  }

  pub fn parse(token_stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    // Consume everything up to the next `=>` or `;`. The two contexts that
    // types appear in - terminal type declaration and production rule return
    // type - are always proceeded by one of these two symbols.

    let first_sym = token_stream.expect_symbol()?;
    let mut meta = first_sym.meta().clone();

    loop {
      let sym = token_stream
        .peek_expect_symbol()
        .intercept("expected type")?;

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

fn parse_name(symbol_stream: &mut impl SymbolStream) -> ParserGeneratorResult<String> {
  Ok("test".into())
}

fn parse_terminal_symbol_type(
  symbol_stream: &mut impl SymbolStream,
) -> ParserGeneratorResult<String> {
  Ok("test".into())
}

pub fn parse_grammar(mut symbol_stream: impl SymbolStream) -> ParserGeneratorResult<Grammar> {
  let grammar_name = parse_name(&mut symbol_stream)?;
  let terminal_symbol_type = parse_terminal_symbol_type(&mut symbol_stream)?;

  todo!();
}
