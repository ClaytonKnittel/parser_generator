use proc_macro2::{Delimiter, Spacing, Span, TokenStream, TokenTree};
use std::fmt::{Display, Formatter};

use crate::error::{InterceptResult, ParserGeneratorError, ParserGeneratorResult};

fn is_leading_ident_char(c: char) -> bool {
  char::is_alphabetic(c) || c == '_'
}

fn is_ident_char(c: char) -> bool {
  char::is_alphanumeric(c) || c == '_'
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
    self.span = self.span.join(*other.span()).unwrap_or(self.span);
    self.tokens.extend(other.tokens.clone());

    Ok(())
  }

  pub fn make_err(&self, message: impl Into<String>) -> ParserGeneratorError {
    ParserGeneratorError::new(message, self.span)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone)]
pub enum SymbolT {
  // Operators are any special symbol, listed above.
  Op(Operator),
  // Identifiers are anything matching [a-zA-Z_]+
  Ident(String),
  // Literals are things that can only be terminals, like single-quote strings
  // (chars). Ident's may also be terminals, depending on what is being parsed.
  Literal(String),
  // Groups are the blocks of code to execute in successful matches.
  Group(proc_macro2::Group),
  // Tuples are blocks of code within parenthesis.
  Tuple(TokenStream),
  // Arrays are for array slice types, i.e. &[u64].
  Array(TokenStream),
}

impl SymbolT {
  pub fn is_op(&self, expected_op: Operator) -> bool {
    match self {
      Self::Op(op) => *op == expected_op,
      _ => false,
    }
  }

  pub fn is_identifier_with_name(&self, expected_name: &str) -> bool {
    match self {
      Self::Ident(name) => name == expected_name,
      _ => false,
    }
  }
}

#[derive(Clone)]
pub struct Symbol {
  sym: SymbolT,
  meta: SymbolMeta,
}

impl Symbol {
  pub fn symbol_type(&self) -> &SymbolT {
    &self.sym
  }

  pub fn meta(&self) -> &SymbolMeta {
    &self.meta
  }

  pub fn take_meta(self) -> SymbolMeta {
    self.meta
  }

  fn from_group(group: proc_macro2::Group, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    let meta = SymbolMeta::new(group.span(), tokens);

    let sym = match group.delimiter() {
      Delimiter::Brace => SymbolT::Group(group),
      Delimiter::Parenthesis => SymbolT::Tuple(group.stream()),
      Delimiter::Bracket => SymbolT::Array(group.stream()),
      Delimiter::None => {
        return Err(ParserGeneratorError::new(
          "Group without delimiter",
          group.span_open(),
        ));
      }
    };

    Ok(Self { sym, meta })
  }

  fn verify_ident_spelling(ident: &proc_macro2::Ident) -> ParserGeneratorResult {
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

  fn from_ident(ident: proc_macro2::Ident, tokens: TokenTree) -> ParserGeneratorResult<Self> {
    Self::verify_ident_spelling(&ident)?;
    Ok(Self {
      sym: SymbolT::Ident(ident.to_string()),
      meta: SymbolMeta::new(ident.span(), tokens),
    })
  }

  fn from_literal(literal: proc_macro2::Literal, tokens: TokenTree) -> ParserGeneratorResult<Self> {
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
      SymbolT::Group(token_stream) => write!(f, "{{{}}}", token_stream),
      SymbolT::Tuple(token_stream) => write!(f, "({})", token_stream),
      SymbolT::Array(token_stream) => write!(f, "[{}]", token_stream),
      SymbolT::Literal(token_stream) => write!(f, "{:?}", token_stream),
    }
  }
}

#[derive(Default)]
enum PunctBuilder {
  #[default]
  Empty,
  PrevWasEq(SymbolMeta),
  PrevWasColon(SymbolMeta),
}

impl PunctBuilder {
  fn consume_next(
    &mut self,
    punct: proc_macro2::Punct,
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
          prev_meta.merge(&meta).intercept("callsite 1")?;
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
          other_meta.merge(&meta).intercept("callsite 2")?;
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

#[derive(Default)]
struct SymbolBuilder {
  punct_builder: PunctBuilder,
}

impl SymbolBuilder {
  fn consume_next_token(&mut self, tokens: TokenTree) -> ParserGeneratorResult<Option<Symbol>> {
    Ok(match tokens.clone() {
      TokenTree::Group(group) => Some(Symbol::from_group(group, tokens)?),
      TokenTree::Ident(ident) => Some(Symbol::from_ident(ident, tokens)?),
      TokenTree::Literal(literal) => Some(Symbol::from_literal(literal, tokens)?),
      TokenTree::Punct(punct) => self.punct_builder.consume_next(punct, tokens)?,
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
    .filter_map(Result::transpose)
}
