use std::{fmt::Display, str::FromStr};

use lr_table::vocabulary::AugmentedVocabToken;
use proc_macro2::{Delimiter, Spacing, Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};

use crate::{
  ParserGeneratorError, ParserGeneratorResult,
  symbol::{Operator, Symbol, SymbolMeta, SymbolT},
  symbol_stream::SymbolStream,
};

fn is_punct(tokens: &TokenTree, ch: char, spacing: Spacing) -> bool {
  if let TokenTree::Punct(punct) = tokens {
    punct.as_char() == ch && punct.spacing() == spacing
  } else {
    false
  }
}

fn is_wildcard(tokens: TokenStream) -> bool {
  let mut iter = tokens.into_iter();
  let Some(first_token) = iter.next() else {
    return false;
  };
  if is_punct(&first_token, '.', Spacing::Joint) {
    if let Some(next_token) = iter.next()
      && is_punct(&next_token, '.', Spacing::Alone)
      && iter.next().is_none()
    {
      true
    } else {
      false
    }
  } else if is_punct(&first_token, '_', Spacing::Alone) {
    iter.next().is_none()
  } else {
    false
  }
}

pub enum PatternMode {
  /// Names the associated data of the pattern being matched using
  /// `UserDefinedSymbol::wildcard_bound_variable()`.
  Named,
  /// Uses placeholder `..`.
  Unnamed,
  /// The construction of a pattern, not the matching. Places `.clone()` after
  /// the named variable.
  Rhs,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum ParenGroup {
  Pattern(String),
  Wildcard,
  None,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UserDefinedSymbol {
  name: String,
  paren_group: ParenGroup,
}

impl UserDefinedSymbol {
  pub const fn wildcard_bound_variable() -> &'static str {
    "wildcard_var"
  }

  pub fn is_wildcard(&self) -> bool {
    matches!(self.paren_group, ParenGroup::Wildcard)
  }

  pub fn as_literal(&self) -> ParserGeneratorResult<proc_macro2::Literal> {
    if !matches!(self.paren_group, ParenGroup::None) {
      return Err(ParserGeneratorError::new(
        "Cannot cast UserDefinedSymbol with pattern to literal",
        Span::call_site(),
      ));
    }
    proc_macro2::Literal::from_str(&self.name)
      .map_err(|err| ParserGeneratorError::from_foreign_error(err, Span::call_site()))
  }

  pub fn as_ident(&self) -> proc_macro2::Ident {
    proc_macro2::Ident::new(&self.name, Span::call_site())
  }

  pub fn as_pattern(&self, mode: PatternMode) -> ParserGeneratorResult<proc_macro2::TokenStream> {
    let ident = self.as_ident();
    Ok(match &self.paren_group {
      ParenGroup::Pattern(pat) => {
        let g = proc_macro2::Group::new(
          Delimiter::Parenthesis,
          proc_macro2::TokenStream::from_str(pat)
            .expect("Failed to re-tokenize `UserDefinedSymbol::pat`"),
        );
        quote! { #ident #g }
      }
      ParenGroup::Wildcard => {
        let bound_var = proc_macro2::Ident::new(Self::wildcard_bound_variable(), Span::call_site());
        match mode {
          PatternMode::Named => quote! { #ident(#bound_var) },
          PatternMode::Unnamed => quote! { #ident(..) },
          PatternMode::Rhs => quote! { #ident(#bound_var.clone()) },
        }
      }
      ParenGroup::None => quote! { #ident },
    })
  }
}

impl Display for UserDefinedSymbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)?;
    match &self.paren_group {
      ParenGroup::Pattern(pat) => write!(f, "({pat})"),
      ParenGroup::Wildcard => write!(f, "(..)"),
      ParenGroup::None => Ok(()),
    }
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TerminalSymbol {
  Symbol(UserDefinedSymbol),
  Epsilon,
}

impl TerminalSymbol {
  pub fn is_epsilon(&self) -> bool {
    matches!(self, Self::Epsilon)
  }
}

pub struct Terminal {
  symbol: TerminalSymbol,
  meta: SymbolMeta,
}

impl Terminal {
  fn new_plain_symbol(text: String, meta: SymbolMeta) -> Self {
    Self {
      symbol: TerminalSymbol::Symbol(UserDefinedSymbol {
        name: text,
        paren_group: ParenGroup::None,
      }),
      meta,
    }
  }

  fn new_symbol_with_pattern(text: String, pattern: String, meta: SymbolMeta) -> Self {
    Self {
      symbol: TerminalSymbol::Symbol(UserDefinedSymbol {
        name: text,
        paren_group: ParenGroup::Pattern(pattern),
      }),
      meta,
    }
  }

  fn new_symbol_with_wildcard(text: String, meta: SymbolMeta) -> Self {
    Self {
      symbol: TerminalSymbol::Symbol(UserDefinedSymbol {
        name: text,
        paren_group: ParenGroup::Wildcard,
      }),
      meta,
    }
  }

  fn new_epsilon(meta: SymbolMeta) -> Self {
    Self {
      symbol: TerminalSymbol::Epsilon,
      meta,
    }
  }

  pub fn symbol(&self) -> &TerminalSymbol {
    &self.symbol
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  fn parse_ident(
    ident_name: String,
    token: Symbol,
    stream: &mut impl SymbolStream,
  ) -> ParserGeneratorResult<Self> {
    let meta = token.take_meta();
    let next_token = stream.peek_expect_symbol()?;
    let SymbolT::Paren(pattern) = next_token.symbol_type() else {
      return Ok(Self::new_plain_symbol(ident_name, meta));
    };

    let pattern = pattern.to_token_stream();
    next_token.take();

    if is_wildcard(pattern.clone()) {
      Ok(Self::new_symbol_with_wildcard(ident_name, meta))
    } else {
      let pattern = pattern.to_string();
      Ok(Self::new_symbol_with_pattern(ident_name, pattern, meta))
    }
  }

  pub fn parse(stream: &mut impl SymbolStream) -> ParserGeneratorResult<Self> {
    let next_token = stream.expect_symbol()?;
    match next_token.symbol_type() {
      SymbolT::Literal(text) => Ok(Self::new_plain_symbol(
        text.to_owned(),
        next_token.take_meta(),
      )),
      SymbolT::Ident(ident) => Self::parse_ident(ident.to_owned(), next_token, stream),
      SymbolT::Op(Operator::Bang) => Ok(Self::new_epsilon(next_token.take_meta())),
      _ => Err(next_token.meta().make_err("Unexpected token")),
    }
  }
}

impl From<TerminalSymbol> for AugmentedVocabToken<UserDefinedSymbol> {
  fn from(value: TerminalSymbol) -> Self {
    match value {
      TerminalSymbol::Symbol(symbol) => Self::Token(symbol),
      TerminalSymbol::Epsilon => Self::Epsilon,
    }
  }
}
