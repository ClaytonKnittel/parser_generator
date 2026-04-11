use std::{error::Error, fmt::Display};

use googletest::prelude::*;
use parser_generator::{ParserUserError, error::ParserError, grammar, parser::ParserNoContext};

#[derive(Clone, Debug, ParserUserError)]
struct MyError {
  message: String,
}

impl Error for MyError {}

impl From<MyOtherError> for MyError {
  fn from(value: MyOtherError) -> Self {
    Self {
      message: value.message,
    }
  }
}

impl Display for MyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

#[derive(Debug)]
struct MyOtherError {
  message: String,
}
impl Error for MyOtherError {}

impl Display for MyOtherError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

#[derive(Clone, Debug)]
struct A(u32);

#[derive(Clone, Debug)]
enum Container {
  A(A),
}

impl Container {
  fn try_from_a(value: A) -> std::result::Result<Container, MyOtherError> {
    if value.0 < 10 {
      Ok(Container::A(value))
    } else {
      Err(MyOtherError {
        message: format!("Value \"{}\" cannot be greater than 9", value.0),
      })
    }
  }
}

impl TryFrom<A> for Container {
  type Error = MyOtherError;

  fn try_from(value: A) -> std::result::Result<Container, Self::Error> {
    Container::try_from_a(value)
  }
}

grammar!(
  name: AutoInto;
  terminal: char;
  error_type: MyError;

  <root>: Container => <a_or_b>;

  <a_or_b>: Container => <a>;
  <a_or_b>: Container => <b>;

  <a>: A => 'a' { A(1) };
  <a>: A => 'z' { A(100) };

  <a>: A => 'b' { A(1).try_into()? };
  <a>: A => 'y' { A(100).try_into()? };

  <b>: Container => 'c' { Container::try_from_a(A(1))? };
  <b>: Container => 'x' { Container::try_from_a(A(100))? };
);

#[gtest]
fn parse_a() {
  let result = AutoInto::parse("a".chars());
  expect_that!(result, ok(pat!(Container::A(pat!(A(eq(&1)))))));

  let result = AutoInto::parse("b".chars());
  expect_that!(result, ok(pat!(Container::A(pat!(A(eq(&1)))))));

  let result = AutoInto::parse("c".chars());
  expect_that!(result, ok(pat!(Container::A(pat!(A(eq(&1)))))));
}

#[gtest]
fn parse_b() {
  let result = AutoInto::parse("z".chars());
  expect_that!(
    result,
    err(pat!(ParserError::UserError(displays_as(
      contains_substring("Value \"100\" cannot be greater than 9")
    ))))
  );

  let result = AutoInto::parse("y".chars());
  expect_that!(
    result,
    err(pat!(ParserError::UserError(displays_as(
      contains_substring("Value \"100\" cannot be greater than 9")
    ))))
  );

  let result = AutoInto::parse("x".chars());
  expect_that!(
    result,
    err(pat!(ParserError::UserError(displays_as(
      contains_substring("Value \"100\" cannot be greater than 9")
    ))))
  );
}
