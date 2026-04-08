use std::{error::Error, fmt::Display};

use googletest::prelude::*;
use parser_generator::{
  error::{ParserError, ParserUserError},
  grammar,
  parser::Parser,
};

#[derive(Clone, Debug)]
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

impl From<std::convert::Infallible> for MyError {
  fn from(value: std::convert::Infallible) -> Self {
    match value {}
  }
}
impl ParserUserError for MyError {}

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

impl TryFrom<A> for Container {
  type Error = MyOtherError;

  fn try_from(value: A) -> std::result::Result<Self, Self::Error> {
    if value.0 < 10 {
      Ok(Container::A(value))
    } else {
      Err(MyOtherError {
        message: format!("Value \"{}\" cannot be greater than 9", value.0),
      })
    }
  }
}

grammar!(
  name: AutoInto;
  terminal: char;
  error_type: MyError;

  <root>: Container => <a>;

  <a>: A => 'a' { A(1) };
  <a>: A => 'z' { A(100) };

  <a>: A => 'b' { A(1).try_into()? };
  <a>: A => 'y' { A(100).try_into()? };
);

#[gtest]
fn parse_a() {
  let result = AutoInto::parse("a".chars());

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
}
