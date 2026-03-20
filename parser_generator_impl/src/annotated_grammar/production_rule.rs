use crate::{
  annotated_grammar::{
    production_node::ProductionNode,
    production_ref::{ProductionRef, ProductionRefName},
    terminal::TerminalSymbol,
    util::expect_symbol_with,
  },
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  ParserGeneratorResult,
};

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
  rule: Vec<ProductionNode>,
  block: Option<Constructor>,
  meta: SymbolMeta,
}

impl ProductionRule {
  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let name = ProductionRef::parse(stream)?;
    let mut meta = name.meta().clone();

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
          meta.merge(symbol.meta());
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
          meta.merge(&semicolon_meta);
          break;
        }
        _ => {}
      }

      rule.push(ProductionNode::parse(stream)?);
    }

    Ok(Self {
      name,
      rule,
      block,
      meta,
    })
  }
}

impl From<ProductionRule> for lr_table::grammar::ProductionRule<TerminalSymbol, ProductionRefName> {
  fn from(value: ProductionRule) -> Self {
    Self::new(
      value.name.name().clone(),
      value.rule.into_iter().map(|node| node.into()).collect(),
    )
  }
}
