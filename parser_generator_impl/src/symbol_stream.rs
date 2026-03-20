use std::{iter::Peekable, ops::Deref};

use proc_macro::Span;

use crate::{
  error::{CloneErr, EraseOk, ParserGeneratorError},
  symbol::Symbol,
  ParserGeneratorResult,
};

pub trait SymbolStream {
  type Iterator: Iterator<Item = ParserGeneratorResult<Symbol>>;

  fn expect_symbol(&mut self) -> ParserGeneratorResult<Symbol>;

  fn peek_expect_symbol(&mut self) -> ParserGeneratorResult<SymbolProxy<'_, Self::Iterator>>;

  fn next(&mut self) -> Option<ParserGeneratorResult<Symbol>>;

  fn peek(&mut self) -> Option<ParserGeneratorResult<SymbolProxy<'_, Self::Iterator>>>;
}

pub struct SymbolStreamImpl<I: Iterator<Item = ParserGeneratorResult<Symbol>>> {
  iter: Peekable<I>,
  current_span: Option<Span>,
}

impl<I: Iterator<Item = ParserGeneratorResult<Symbol>>> SymbolStreamImpl<I> {
  pub fn new(iter: I) -> Self {
    Self {
      iter: iter.peekable(),
      current_span: None,
    }
  }

  fn current_span(&self) -> Option<Span> {
    self.current_span
  }
}

impl<I: Iterator<Item = ParserGeneratorResult<Symbol>>> SymbolStream for SymbolStreamImpl<I> {
  type Iterator = I;

  fn expect_symbol(&mut self) -> ParserGeneratorResult<Symbol> {
    let prev_span = self.current_span();
    self
      .iter
      .next()
      .ok_or_else(|| {
        ParserGeneratorError::new(
          "Unexpected end of input",
          prev_span.unwrap_or(Span::call_site()),
        )
      })
      .flatten()
  }

  fn peek_expect_symbol(&mut self) -> ParserGeneratorResult<SymbolProxy<'_, I>> {
    let prev_span = self.current_span();
    self
      .peek()
      .ok_or_else(|| {
        ParserGeneratorError::new(
          "Unexpected end of input",
          prev_span.unwrap_or(Span::call_site()),
        )
      })
      .flatten()
  }

  fn next(&mut self) -> Option<ParserGeneratorResult<Symbol>> {
    let next_symbol = self.iter.next();
    self.current_span = match &next_symbol {
      Some(Ok(symbol_result)) => Some(*symbol_result.meta().span()),
      _ => None,
    };
    next_symbol
  }

  fn peek(&mut self) -> Option<ParserGeneratorResult<SymbolProxy<'_, I>>> {
    let symbol_result = self.iter.peek()?;
    Some(
      symbol_result
        .clone_err()
        .erase_ok()
        .map(|_| SymbolProxy::new(self)),
    )
  }
}

/// A type returned by `peek` methods from `SymbolStreamImpl` when the peeked
/// value is present. Can be consumed with `take()` to advance the symbol
/// stream, returning the symbol without needing `unwrap()`.
pub struct SymbolProxy<'a, I: Iterator<Item = ParserGeneratorResult<Symbol>>> {
  iter: &'a mut SymbolStreamImpl<I>,
  symbol: Symbol,
}

impl<'a, I: Iterator<Item = ParserGeneratorResult<Symbol>>> SymbolProxy<'a, I> {
  fn new(iter: &'a mut SymbolStreamImpl<I>) -> Self {
    debug_assert!(iter.iter.peek().is_some_and(Result::is_ok));
    let symbol = unsafe {
      iter
        .iter
        .peek()
        .unwrap_unchecked()
        .as_ref()
        .unwrap_unchecked()
    }
    .clone();
    Self { iter, symbol }
  }

  /// Advances the symbol stream and returns the peeked symbol.
  pub fn take(self) -> Symbol {
    let next = self.iter.next();
    debug_assert!(next.is_some_and(|next| next.is_ok()));
    self.symbol
  }
}

impl<'a, I: Iterator<Item = ParserGeneratorResult<Symbol>>> Deref for SymbolProxy<'a, I> {
  type Target = Symbol;

  fn deref(&self) -> &Symbol {
    &self.symbol
  }
}
