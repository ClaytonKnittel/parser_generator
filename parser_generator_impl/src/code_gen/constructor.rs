use std::collections::{hash_map::Entry, HashMap};

use lr_table::grammar::ProductionRuleIndex;
use proc_macro2::{Delimiter, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use syn::spanned::Spanned;

use crate::{
  annotated_grammar::{
    parse_grammar::GrammarInfo,
    production_node::ProductionNode,
    production_ref::ProductionRefName,
    production_rule::{Constructor, ProductionRule},
  },
  code_gen::{reduce_rule::bound_variable_ident, util::TokenStreamResult},
  ParserGeneratorError, ParserGeneratorResult,
};

fn generate_default_constructor(rule: &ProductionRule) -> TokenStreamResult {
  if rule.rule().len() != 1 {
    return Err(
      rule
        .meta()
        .make_err("Must provide a constructor for rules with more than 1 node"),
    );
  }

  let var = bound_variable_ident(0);
  Ok(quote! {
    { #var }
  })
}

struct SubstitutionMap {
  production_label_map: HashMap<ProductionRefName, usize>,
  num_nodes: usize,
}

impl SubstitutionMap {
  const DUPLICATE_SYMBOL: usize = usize::MAX;

  fn build(rule: &[ProductionNode]) -> Self {
    let num_nodes = rule.len();
    let mut production_label_map = HashMap::new();
    for (idx, node) in rule.iter().enumerate() {
      match node {
        ProductionNode::Production(production) => {
          match production_label_map.entry(production.name().clone()) {
            Entry::Occupied(mut entry) => {
              *entry.get_mut() = Self::DUPLICATE_SYMBOL;
            }
            Entry::Vacant(entry) => {
              entry.insert(idx);
            }
          }
        }
        ProductionNode::Terminal(_) => {}
      }
    }

    Self {
      production_label_map,
      num_nodes,
    }
  }

  fn lookup_label(&self, ident: &syn::Ident) -> ParserGeneratorResult<usize> {
    let label = ident.to_string();
    let index = *self
      .production_label_map
      .get(&ProductionRefName(label))
      .ok_or_else(|| {
        ParserGeneratorError::new(
          "Unrecognized production label: not found in rule",
          ident.span(),
        )
      })?;

    if index == Self::DUPLICATE_SYMBOL {
      return Err(ParserGeneratorError::new(
        "Cannot refer to production by name, as it appears multiple times in \
         the rule. Use `#n` instead, where `n` is the index in the rule.",
        ident.span(),
      ));
    }

    Ok(index)
  }

  fn literal_to_index(
    &self,
    literal: &proc_macro2::Literal,
    rule: &[ProductionNode],
  ) -> ParserGeneratorResult<usize> {
    let index = literal
      .to_string()
      .parse::<usize>()
      .map_err(|err| ParserGeneratorError::from_foreign_error(err, literal.span()))?;

    if index >= self.num_nodes {
      return Err(ParserGeneratorError::new(
        format!(
          "Index is out of range of rules, expected within [0, {})",
          self.num_nodes
        ),
        literal.span(),
      ));
    } else if rule[index].is_epsilon() {
      return Err(ParserGeneratorError::new(
        format!("Index {} refers to `!` which matches nothing", index),
        literal.span(),
      ));
    }

    Ok(index)
  }

  fn substitute_at(
    &self,
    iter: &mut impl Iterator<Item = TokenTree>,
    rule: &[ProductionNode],
    rule_span: Span,
  ) -> ParserGeneratorResult<impl Into<TokenTree>> {
    let index = match iter.next() {
      Some(TokenTree::Ident(ident)) => self.lookup_label(&ident)?,
      Some(TokenTree::Literal(literal)) => self.literal_to_index(&literal, rule)?,
      _ => {
        return Err(ParserGeneratorError::new(
          "`#` must be followed by an identifier or index",
          rule_span,
        ));
      }
    };

    Ok(bound_variable_ident(index))
  }

  fn substitute_vars(&self, stream: TokenStream, rule: &[ProductionNode]) -> TokenStreamResult {
    let mut iter = stream.into_iter();
    let mut res = TokenStream::new();

    while let Some(token) = iter.next() {
      let rule_span = token.span();

      match token {
        TokenTree::Group(group) => {
          res.append(proc_macro2::Group::new(
            group.delimiter(),
            self.substitute_vars(group.stream(), rule)?,
          ));
        }
        TokenTree::Punct(punct) => {
          if punct.spacing() == Spacing::Alone && punct.as_char() == '#' {
            res.append(self.substitute_at(&mut iter, rule, rule_span)?);
          } else {
            res.append(TokenTree::Punct(punct));
          }
        }
        TokenTree::Ident(_) | TokenTree::Literal(_) => {
          res.append(token);
        }
      }
    }

    Ok(res)
  }
}

fn rewrite_provided_constructor(
  rule: &[ProductionNode],
  constructor: &Constructor,
) -> TokenStreamResult {
  if constructor.body().delimiter() != Delimiter::Brace {
    return Err(ParserGeneratorError::new(
      "Constructor must be delimited by \"{}\"",
      constructor.body().delim_span().span(),
    ));
  }
  let subsitution_map = SubstitutionMap::build(rule);

  let body = proc_macro2::Group::new(
    Delimiter::Brace,
    subsitution_map.substitute_vars(constructor.body().stream(), rule)?,
  );

  Ok(quote! { #body })
}

pub fn build_constructor(
  production_rule: ProductionRuleIndex,
  grammar_info: &GrammarInfo,
) -> TokenStreamResult {
  let rule = grammar_info.production_rule(production_rule);
  match rule.constructor() {
    Some(constructor) => rewrite_provided_constructor(rule.rule(), constructor),
    None => {
      if rule.return_type().is_some() {
        generate_default_constructor(rule)
      } else {
        Ok(quote! { () })
      }
    }
  }
}
