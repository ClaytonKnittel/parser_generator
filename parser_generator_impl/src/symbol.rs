use proc_macro::TokenTree::{self, Group, Ident, Literal, Punct};
use proc_macro::{Delimiter, Spacing, Span, TokenStream};
use std::fmt::{Display, Formatter};

use crate::error::{ParserGeneratorError, ParserGeneratorResult};

fn is_leading_ident_char(c: char) -> bool {
  return char::is_alphabetic(c) || c == '_';
}

fn is_ident_char(c: char) -> bool {
  return char::is_alphanumeric(c) || c == '_';
}

fn string_is_identifier(string: &str) -> bool {
  string.starts_with(is_leading_ident_char) && string.chars().all(is_ident_char)
}

#[derive(Clone)]
pub struct SymbolMeta {
  span: Span,
  tokens: TokenStream,
}

impl SymbolMeta {
  pub fn new(span: Span, tokens: impl Into<TokenStream>) -> Self {
    Self {
      span,
      tokens: tokens.into(),
    }
  }

  pub fn span(&self) -> &Span {
    &self.span
  }

  pub fn tokens(&self) -> &TokenStream {
    &self.tokens
  }

  pub fn merge(&mut self, other: &Self) -> ParserGeneratorResult {
    self.span = self
      .span
      .join(*other.span())
      .ok_or_else(|| other.make_err("Failed to join spans of adjacent puncts"))?;
    self.tokens.extend(other.tokens.clone());

    Ok(())
  }

  pub fn make_err(&self, message: impl Into<String>) -> ParserGeneratorError {
    ParserGeneratorError::new(message.into(), self.span.clone())
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
  // =>
  Arrow,
  // :
  Colon,
  // ;
  Semicolon,
  // #
  NumberSign,
  // <
  BeginProd,
  // >
  EndProd,
  // |
  Pipe,
  // ::
  Scope,
  // !
  Bang,
}

impl Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Arrow => "=>",
        Self::Colon => ":",
        Self::Semicolon => ";",
        Self::NumberSign => "#",
        Self::BeginProd => "<",
        Self::EndProd => ">",
        Self::Pipe => "|",
        Self::Scope => "::",
        Self::Bang => "!",
      }
    )
  }
}

pub enum SymbolT {
  // Operators are any special symbol, listed above.
  Op(Operator),
  // Identifiers are anything matching [a-zA-Z_]+
  Ident(String),
  // Literals are things that can only be terminals, like single-quote strings
  // (chars). Ident's may also be terminals, depending on what is being parsed.
  Literal(String),
  // Groups are the blocks of code to execute in successful matches.
  Group(proc_macro::Group),
  // Tuples are blocks of code within parenthesis.
  Tuple(TokenStream),
  // Arrays are for array slice types, i.e. &[u64].
  Array(TokenStream),
}

pub struct Symbol {
  pub sym: SymbolT,
  pub meta: SymbolMeta,
}

impl Symbol {
  fn from_group(group: proc_macro::Group, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    let meta = SymbolMeta::new(group.span(), tokens);

    let sym = match group.delimiter() {
      Delimiter::Brace => SymbolT::Group(group),
      Delimiter::Parenthesis => SymbolT::Tuple(group.stream()),
      Delimiter::Bracket => SymbolT::Array(group.stream()),
      Delimiter::None => {
        return Err(ParserGeneratorError::new(
          "Group without delimiter",
          group.span_open(),
        ))
      }
    };

    Ok(Self { sym, meta })
  }

  fn verify_ident_spelling(ident: &proc_macro::Ident) -> ParserGeneratorResult {
    let ident_str = ident.to_string();
    if string_is_identifier(&ident_str) {
      Ok(())
    } else {
      Err(ParserGeneratorError::new(
        format!(
          "Identifier \"{ident_str}\" is not alphanumeric/_ with a non-numeric leading character"
        ),
        ident.span(),
      ))
    }
  }

  fn from_ident(ident: proc_macro::Ident, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    Self::verify_ident_spelling(&ident)?;
    Ok(Self {
      sym: SymbolT::Ident(ident.to_string()),
      meta: SymbolMeta::new(ident.span(), tokens),
    })
  }

  fn from_literal(literal: proc_macro::Literal, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    let meta = SymbolMeta::new(literal.span(), tokens);

    let literal_str = literal.to_string();
    if literal_str.starts_with('\'') {
      Ok(Self {
        sym: SymbolT::Literal(literal_str),
        meta,
      })
    } else if string_is_identifier(&literal_str) {
      Ok(Self {
        sym: SymbolT::Ident(literal_str),
        meta,
      })
    } else {
      Err(meta.make_err(format!("Unrecognized literal: {literal_str}")))
    }
  }

  fn as_punctuation(operator: Operator, meta: SymbolMeta) -> Self {
    Self {
      sym: SymbolT::Op(operator),
      meta,
    }
  }
}

impl Display for Symbol {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match &self.sym {
      SymbolT::Op(op) => write!(f, "{:?}", op),
      SymbolT::Ident(ident) => write!(f, "<{}>", ident),
      SymbolT::Group(token_stream) => write!(f, "{}", token_stream),
      SymbolT::Tuple(token_stream) => write!(f, "({})", token_stream),
      SymbolT::Array(token_stream) => write!(f, "[{}]", token_stream),
      SymbolT::Literal(token_stream) => write!(f, "{:?}", token_stream),
    }
  }
}

enum PunctBuilder {
  Empty,
  PrevWasEq(SymbolMeta),
  PrevWasColon(SymbolMeta),
}

impl PunctBuilder {
  fn consume_next(
    &mut self,
    punct: proc_macro::Punct,
    tokens: TokenTree,
  ) -> ParserGeneratorResult<Option<Symbol>> {
    let meta = SymbolMeta::new(punct.span(), tokens);

    Ok(match self {
      Self::Empty => match punct.as_char() {
        ';' => Some(Symbol::as_punctuation(Operator::Semicolon, meta)),
        '#' => Some(Symbol::as_punctuation(Operator::NumberSign, meta)),
        '<' => Some(Symbol::as_punctuation(Operator::BeginProd, meta)),
        '>' => Some(Symbol::as_punctuation(Operator::EndProd, meta)),
        '|' => Some(Symbol::as_punctuation(Operator::Pipe, meta)),
        '!' => Some(Symbol::as_punctuation(Operator::Bang, meta)),
        '=' => match punct.spacing() {
          Spacing::Joint => {
            *self = PunctBuilder::PrevWasEq(meta);
            None
          }
          Spacing::Alone => return Err(meta.make_err("Unexpected token \"=\"")),
        },
        ':' => match punct.spacing() {
          Spacing::Joint => {
            *self = PunctBuilder::PrevWasColon(meta);
            None
          }
          Spacing::Alone => Some(Symbol::as_punctuation(Operator::Colon, meta)),
        },
        _ => {
          return Err(meta.make_err(format!("Unexpected token \"{}\"", punct.as_char())));
        }
      },
      Self::PrevWasEq(prev_meta) => match punct.as_char() {
        '>' => {
          prev_meta.merge(&meta)?;
          let symbol = Symbol::as_punctuation(Operator::Arrow, prev_meta.clone());
          *self = PunctBuilder::Empty;
          Some(symbol)
        }
        _ => {
          return Err(meta.make_err(format!("Unexpected token \"={}\"", punct.as_char())));
        }
      },
      Self::PrevWasColon(other_meta) => match punct.as_char() {
        ':' => {
          other_meta.merge(&meta)?;
          let symbol = Symbol::as_punctuation(Operator::Scope, other_meta.clone());
          *self = PunctBuilder::Empty;
          Some(symbol)
        }
        _ => {
          return Err(meta.make_err(format!("Unexpected token \":{}\"", punct.as_char())));
        }
      },
    })
  }
}

impl Default for PunctBuilder {
  fn default() -> Self {
    Self::Empty
  }
}

#[derive(Default)]
struct SymbolBuilder {
  punct_builder: PunctBuilder,
}

impl SymbolBuilder {
  fn consume_next_token(&mut self, tokens: TokenTree) -> ParserGeneratorResult<Option<Symbol>> {
    Ok(match tokens.clone() {
      Group(group) => Some(Symbol::from_group(group, tokens)?),
      Ident(ident) => Some(Symbol::from_ident(ident, tokens)?),
      Literal(literal) => Some(Symbol::from_literal(literal, tokens)?),
      Punct(punct) => self.punct_builder.consume_next(punct, tokens)?,
    })
  }
}

pub fn tokenize_from_stream(
  tokens: TokenStream,
) -> impl Iterator<Item = ParserGeneratorResult<Symbol>> {
  tokens
    .into_iter()
    .scan(SymbolBuilder::default(), |symbol_builder, tokens| {
      Some(symbol_builder.consume_next_token(tokens))
    })
    .map(Result::transpose)
    .flatten()
}
