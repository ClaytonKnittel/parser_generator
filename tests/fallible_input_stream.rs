use std::{error::Error, fmt::Display};

use googletest::prelude::*;
use parser_generator::{ParserUserError, parser::ParserNoContext};

#[derive(Clone, Debug, ParserUserError)]
struct MyError;

impl Display for MyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for MyError {}

parser_generator::grammar! {
  name: Test;
  terminal: char;
  error_type: MyError;

  <S>: u32 => <A> '+' <B> {
    #A + #B
  };
  <A>: u32 => <dig> { u32::from(#dig) - u32::from(b'0') };
  <B>: u32 => <dig> { u32::from(#dig) - u32::from(b'0') };
  <dig>: char =>
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
}

#[gtest]
fn test_stream_fails() {
  let iter = [Ok('3'), Ok('+'), Err(MyError)];
  expect_that!(
    Test::parse_fallible(iter),
    err(displays_as(contains_substring("MyError")))
  );
}

#[gtest]
fn test_stream_succeeds() {
  let iter = [Ok::<_, MyError>('3'), Ok('+'), Ok('2')];
  expect_that!(Test::parse_fallible(iter), ok(eq(&5)));
}
