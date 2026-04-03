use googletest::prelude::*;
use parser_generator::{grammar, parser::Parser};

#[derive(Clone, Debug)]
struct A(u32);

#[derive(Clone, Debug)]
struct B(u32);

#[derive(Clone, Debug)]
enum AOrB {
  A(A),
  B(B),
}

impl From<A> for AOrB {
  fn from(value: A) -> Self {
    Self::A(value)
  }
}

impl From<B> for AOrB {
  fn from(value: B) -> Self {
    Self::B(value)
  }
}

grammar!(
  name: AutoInto;
  terminal: char;

  <root>: AOrB => <a_or_b>;

  <a_or_b>: AOrB => <a> { #0.into() };
  <a_or_b>: AOrB => <b> { #0.into() };

  <a>: A => 'a' { A(1) };
  <b>: B => 'b' { B(2) };
);

#[gtest]
fn parse_a() {
  let result = AutoInto::parse("a".chars());

  expect_that!(result, ok(pat!(AOrB::A(pat!(A(eq(&1)))))));
}

#[gtest]
fn parse_b() {
  let result = AutoInto::parse("b".chars());

  expect_that!(result, ok(pat!(AOrB::B(pat!(B(eq(&2)))))));
}
