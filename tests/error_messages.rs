use googletest::prelude::*;
use parser_generator::{error::ParserError, grammar, parser::Parser};

#[derive(Clone, Debug)]
enum Token {
  A(usize),
  B(usize),
  C(usize),
}

grammar!(
  name: AOrB;
  enum_terminal: Token;

  <root> => <text>;

  <text> => <all_a>;
  <text> => <all_b>;

  <all_a> => <a>;
  <all_a> => <all_a> <a>;
  <all_b> => <b>;
  <all_b> => <all_b> <b>;

  <a> => A(..);
  <b> => B(..);
);

fn to_token_stream(text: &str) -> impl Iterator<Item = Token> {
  text.chars().enumerate().map(|(idx, c)| match c {
    'a' => Token::A(idx),
    'b' => Token::B(idx),
    _ => Token::C(idx),
  })
}

#[gtest]
fn unexpected_token() {
  let result = AOrB::parse(to_token_stream("ab"));

  expect_that!(
    result,
    err(pat!(ParserError::ParseError {
      next_token: some(pat!(Token::B(eq(&1))))
    }))
  );
}

#[gtest]
fn unexpected_token_mid_stream() {
  let result = AOrB::parse(to_token_stream("aaaabaa"));

  expect_that!(
    result,
    err(pat!(ParserError::ParseError {
      next_token: some(pat!(Token::B(eq(&4))))
    }))
  );
}

#[gtest]
fn unexpected_token_not_in_vocab() {
  let result = AOrB::parse(to_token_stream("aaaca"));

  expect_that!(
    result,
    err(pat!(ParserError::ParseError {
      next_token: some(pat!(Token::C(eq(&3))))
    }))
  );
}
