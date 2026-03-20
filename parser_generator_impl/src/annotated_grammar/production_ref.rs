use crate::{
  annotated_grammar::util::expect_symbol_with,
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

#[derive(Clone, PartialEq, Eq)]
pub struct ProductionRefName(String);

pub struct ProductionRef {
  name: ProductionRefName,
  meta: SymbolMeta,
}

impl ProductionRef {
  pub fn name(&self) -> &ProductionRefName {
    &self.name
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let mut meta = expect_symbol_with(
      stream,
      |sym| sym.is_op(Operator::BeginProd),
      format!("Expected production name to begin with '<'."),
    )?;

    let prod_name_sym = stream.expect_symbol()?;
    let name = match prod_name_sym.symbol_type() {
      SymbolT::Ident(ident) => Ok(ProductionRefName(ident.to_owned())),
      _ => return Err(prod_name_sym.meta().make_err("Expected production name.")),
    }?;
    meta.merge(prod_name_sym.meta());

    let end_meta = expect_symbol_with(
      stream,
      |sym| sym.is_op(Operator::EndProd),
      format!("Expected production name to end with '>'."),
    )?;
    meta.merge(&end_meta);

    Ok(Self { name, meta })
  }
}
