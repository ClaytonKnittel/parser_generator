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

impl Operator {
  pub fn should_separate(prev_chars: &str, next_char: char) -> bool {
    let mut chars = prev_chars.chars();
    match chars.next() {
      Some('=') => chars.next().is_some() || next_char != '>',
      Some(';') => true,
      Some('#') => true,
      Some('<') => true,
      Some('>') => true,
      Some('|') => true,
      Some(':') => chars.next().is_some() || next_char != ':',
      Some('!') => true,
      Some(_) => false,
      None => true,
    }
  }
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
  pub span: Span,
  pub tokens: TokenStream,
}

impl Symbol {
  fn from_group(group: proc_macro::Group, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    let span = group.span();
    let tokens = tokens.into();

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

    Ok(Self { sym, span, tokens })
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
      span: ident.span(),
      tokens: tokens.into(),
    })
  }

  fn from_literal(literal: proc_macro::Literal, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    let span = literal.span();
    let tokens = tokens.into();

    let literal_str = literal.to_string();
    if literal_str.starts_with('\'') {
      Ok(Self {
        sym: SymbolT::Literal(literal_str),
        span,
        tokens,
      })
    } else if string_is_identifier(&literal_str) {
      Ok(Self {
        sym: SymbolT::Ident(literal_str),
        span,
        tokens,
      })
    } else {
      Err(ParserGeneratorError::new(
        format!("Unrecognized literal: {literal_str}"),
        span,
      ))
    }
  }

  fn as_punctuation(operator: Operator, span: Span, tokens: TokenStream) -> Self {
    Self {
      sym: SymbolT::Op(operator),
      span,
      tokens,
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

struct SymbolMeta {
  span: Span,
  tokens: TokenStream,
}

impl SymbolMeta {
  fn new(span: Span, tokens: TokenStream) -> Self {
    Self { span, tokens }
  }

  fn merge(&mut self, span: Span, tokens: TokenStream) -> ParserGeneratorResult {
    self.span = self
      .span
      .join(span)
      .ok_or_else(|| ParserGeneratorError::new("Failed to join spans of adjacent puncts", span))?;
    self.tokens.extend(TokenStream::from(tokens));

    Ok(())
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
    let span = punct.span();
    let tokens = tokens.into();

    Ok(match self {
      Self::Empty => match punct.as_char() {
        ';' => Some(Symbol::as_punctuation(Operator::Semicolon, span, tokens)),
        '#' => Some(Symbol::as_punctuation(Operator::NumberSign, span, tokens)),
        '<' => Some(Symbol::as_punctuation(Operator::BeginProd, span, tokens)),
        '>' => Some(Symbol::as_punctuation(Operator::EndProd, span, tokens)),
        '|' => Some(Symbol::as_punctuation(Operator::Pipe, span, tokens)),
        '!' => Some(Symbol::as_punctuation(Operator::Bang, span, tokens)),
        '=' => match punct.spacing() {
          Spacing::Joint => {
            *self = PunctBuilder::PrevWasEq(SymbolMeta::new(span, tokens));
            None
          }
          Spacing::Alone => return Err(ParserGeneratorError::new("Unexpected token \"=\"", span)),
        },
        ':' => match punct.spacing() {
          Spacing::Joint => {
            *self = PunctBuilder::PrevWasColon(SymbolMeta::new(span, tokens));
            None
          }
          Spacing::Alone => Some(Symbol::as_punctuation(Operator::Colon, span, tokens)),
        },
        _ => {
          return Err(ParserGeneratorError::new(
            format!("Unexpected token \"{}\"", punct.as_char()),
            span,
          ));
        }
      },
      Self::PrevWasEq(meta) => match punct.as_char() {
        '>' => {
          meta.merge(span, tokens)?;
          let symbol = Symbol::as_punctuation(Operator::Arrow, meta.span, meta.tokens.clone());
          *self = PunctBuilder::Empty;
          Some(symbol)
        }
        _ => {
          return Err(ParserGeneratorError::new(
            format!("Unexpected token \"={}\"", punct.as_char()),
            span,
          ));
        }
      },
      Self::PrevWasColon(meta) => match punct.as_char() {
        '>' => {
          meta.merge(span, tokens)?;
          let symbol = Symbol::as_punctuation(Operator::Arrow, meta.span, meta.tokens.clone());
          *self = PunctBuilder::Empty;
          Some(symbol)
        }
        _ => {
          return Err(ParserGeneratorError::new(
            format!("Unexpected token \":{}\"", punct.as_char()),
            span,
          ));
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

pub fn tokenize_from_stream(tokens: TokenStream) -> ParserGeneratorResult<Vec<Symbol>> {
  let (list, _maybe_prev_chars_and_tokens) = tokens.into_iter().try_fold(
    (Vec::new(), SymbolBuilder::default()),
    |(mut syms, mut symbol_builder), tokens| {
      if let Some(symbol) = symbol_builder.consume_next_token(tokens)? {
        syms.push(symbol);
      }
      Ok((syms, symbol_builder))
    },
  )?;

  return Ok(list);
}
