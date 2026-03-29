use crate::{
  ParserGeneratorResult,
  annotated_grammar::{
    label_type_map::LabelTypeMap,
    production_node::ProductionNode,
    production_ref::{ProductionRef, ProductionRefName},
    util::expect_symbol_with,
  },
  symbol::{Operator, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
  type_symbol::Type,
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

pub struct Constructor {
  group: proc_macro2::Group,
  #[allow(unused)]
  meta: SymbolMeta,
}

impl Constructor {
  fn new(group: proc_macro2::Group, meta: SymbolMeta) -> Self {
    Self { group, meta }
  }

  pub fn body(&self) -> &proc_macro2::Group {
    &self.group
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
  ) -> lr_table::grammar::ProductionRule<String, ProductionRefName> {
    lr_table::grammar::ProductionRule::new(
      self.name.name().clone(),
      self.rule.iter().map(ProductionNode::to_lr_node).collect(),
    )
  }

  pub fn return_type(&self) -> Option<&Type> {
    self.return_type.as_ref()
  }

  pub fn rule(&self) -> &[ProductionNode] {
    &self.rule
  }

  pub fn constructor(&self) -> Option<&Constructor> {
    self.block.as_ref()
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  fn parse_rule(
    stream: &mut impl SymbolStream,
    name: ProductionRef,
    return_type: Option<Type>,
  ) -> ParserGeneratorResult<Self> {
    let first_node = ProductionNode::parse(stream)?;
    let mut meta = first_node.meta().clone();

    let mut rule = vec![first_node];
    let mut block = None;
    loop {
      let next_symbol = stream.peek_expect_symbol()?;
      match next_symbol.symbol_type() {
        SymbolT::Op(Operator::Semicolon | Operator::Pipe) => break,
        SymbolT::Group(group) => {
          let group = group.clone();
          let symbol = next_symbol.take();
          block = Some(Constructor::new(group, symbol.take_meta()));
          break;
        }
        _ => {}
      }

      let node = ProductionNode::parse(stream)?;
      meta.merge(node.meta())?;
      rule.push(node);
    }

    if rule.len() > 1
      && let Some(epsilon) = rule.iter().find(|node| node.is_epsilon())
    {
      return Err(
        epsilon
          .meta()
          .make_err("Epsilon may only appear in a rule by itself"),
      );
    }

    Ok(Self {
      name,
      return_type,
      rule,
      block,
      meta,
    })
  }

  pub fn parse(
    stream: &mut impl SymbolStream,
    label_map: &mut LabelTypeMap,
  ) -> ParserGeneratorResult<Vec<Self>> {
    let name = ProductionRef::parse(stream)?;

    let return_type = maybe_parse_return_type(stream)?;
    label_map.add(name.name().clone(), return_type.clone())?;

    expect_symbol_with(
      stream,
      |symbol| symbol.is_op(Operator::Arrow),
      "Expected `=>` to follow production name",
    )?;

    let first_rule = Self::parse_rule(stream, name.clone(), return_type.clone())?;

    let mut rules = vec![first_rule];
    loop {
      let next_symbol = stream.expect_symbol()?;
      match next_symbol.symbol_type() {
        SymbolT::Op(Operator::Pipe) => {}
        SymbolT::Op(Operator::Semicolon) => break,
        _ => {
          return Err(next_symbol.meta().make_err("Expected either `;` or `|`"));
        }
      }

      rules.push(Self::parse_rule(stream, name.clone(), return_type.clone())?);
    }

    Ok(rules)
  }
}
