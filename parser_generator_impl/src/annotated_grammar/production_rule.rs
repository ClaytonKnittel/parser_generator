use crate::{
  annotated_grammar::{
    production_node::ProductionNode,
    production_ref::{ProductionRef, ProductionRefName},
    terminal::TerminalSymbol,
    util::expect_symbol_with,
  },
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  type_symbol::Type,
  ParserGeneratorResult,
};

fn maybe_parse_return_type(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Option<Type>> {
  let next_symbol = stream.peek_expect_symbol()?;
  if let SymbolT::Op(Operator::Colon) = next_symbol.symbol_type() {
    next_symbol.take();
    Ok(Some(Type::parse(stream)?))
  } else {
    Ok(None)
  }
}

struct Constructor {
  group: proc_macro::Group,
  meta: SymbolMeta,
}

impl Constructor {
  fn new(group: proc_macro::Group, meta: SymbolMeta) -> Self {
    Self { group, meta }
  }
}

pub struct ProductionRule {
  name: ProductionRef,
  return_type: Option<Type>,
  rule: Vec<ProductionNode>,
  block: Option<Constructor>,
  meta: SymbolMeta,
}

impl ProductionRule {
  pub fn to_lr_production_rule(
    &self,
  ) -> lr_table::grammar::ProductionRule<TerminalSymbol, ProductionRefName> {
    lr_table::grammar::ProductionRule::new(
      self.name.name().clone(),
      self.rule.iter().map(ProductionNode::to_lr_node).collect(),
    )
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let name = ProductionRef::parse(stream)?;
    let mut meta = name.meta().clone();

    let return_type = maybe_parse_return_type(stream)?;

    expect_symbol_with(
      stream,
      |symbol| symbol.is_op(Operator::Arrow),
      "Expected `=>` to follow production name",
    )?;

    let mut rule = Vec::new();
    let mut block = None;
    loop {
      let next_symbol = stream.peek_expect_symbol()?;
      match next_symbol.symbol_type() {
        SymbolT::Op(Operator::Semicolon) => {
          let symbol = next_symbol.take();
          meta.merge(symbol.meta())?;
          break;
        }
        SymbolT::Group(group) => {
          let group = group.clone();
          let symbol = next_symbol.take();
          block = Some(Constructor::new(group, symbol.take_meta()));

          let semicolon_meta = expect_symbol_with(
            stream,
            |symbol| symbol.is_op(Operator::Semicolon),
            "Expected `;` to follow rule constructor",
          )?;
          meta.merge(&semicolon_meta)?;
          break;
        }
        _ => {}
      }

      rule.push(ProductionNode::parse(stream)?);
    }

    Ok(Self {
      name,
      return_type,
      rule,
      block,
      meta,
    })
  }
}
