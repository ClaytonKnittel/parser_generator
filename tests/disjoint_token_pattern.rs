use googletest::prelude::*;
use parser_generator::{grammar, parser::ParserNoContext};

#[derive(Clone, Debug)]
struct A {
  field1: u32,
  field2: u32,
}

#[derive(Clone, Debug)]
enum Token {
  A(A),
}

grammar! {
  name: DisjointPattern;
  enum_terminal: Token;

  <root> => <ambiguous>;
  <ambiguous> => A(A { field1: 0..10, field2: _ });
  <ambiguous> => A(A { field1: 5..12, field2: 100 });
}

#[gtest]
fn no_conflicts() {
  // The runtime check for token resolution conflict only triggers if a token
  // matches more than one rule.
  let res = DisjointPattern::parse([Token::A(A {
    field1: 0,
    field2: 100,
  })]);
  expect_that!(res, ok(()));

  let res = DisjointPattern::parse([Token::A(A {
    field1: 8,
    field2: 99,
  })]);
  expect_that!(res, ok(()));
}

#[gtest]
fn conflict() {
  // The runtime check for token resolution conflict only triggers if a token
  // matches more than one rule.
  let res = DisjointPattern::parse([Token::A(A {
    field1: 9,
    field2: 100,
  })]);

  #[cfg(debug_assertions)]
  expect_that!(res, err(anything()));
  #[cfg(not(debug_assertions))]
  expect_that!(res, ok(()));
}
