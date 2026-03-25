use googletest::prelude::*;
use parser_generator::{grammar, parser::Parser};

grammar! {
  name: TestSimple;
  terminal: char;

  <S> => 'a';
}

#[gtest]
fn test_simple_parses() {
  expect_that!(TestSimple::parse("a".chars()), ok(()));
}

#[gtest]
fn test_simple_empty_input() {
  expect_that!(TestSimple::parse("".chars()), err(anything()));
}

#[gtest]
fn test_simple_incorrect_input() {
  expect_that!(TestSimple::parse("b".chars()), err(anything()));
}

#[gtest]
fn test_simple_extra_input() {
  todo!();
  expect_that!(TestSimple::parse("ab".chars()), ok(()));
}

parser_generator_impl::grammar! {
  name: AddMul;
  terminal: char;

  <S>: u32 => <A> { #A };
  <A>: u32 => <A> '+' <P> {
    #A + #P
  } | <P> {
    #P
  };
  <P>: u32 => <P> '*' <V> {
    #P * #V
  } | <V> {
    #V
  };
  <V>: u32 => '2' { 2 } | '3' { 3 };
}

#[gtest]
fn test_add_mul_single_digit() {
  expect_that!(AddMul::parse("2".chars()), ok(eq(&2)));
}

#[gtest]
fn test_add_mul_add() {
  expect_that!(AddMul::parse("2+3".chars()), ok(eq(&5)));
}

#[gtest]
fn test_add_mul_mul() {
  expect_that!(AddMul::parse("2*3".chars()), ok(eq(&6)));
}

#[gtest]
fn test_add_mul_mul_add() {
  expect_that!(AddMul::parse("2*2+3".chars()), ok(eq(&7)));
}

#[gtest]
fn test_add_mul_add_mul() {
  expect_that!(AddMul::parse("2+2*3".chars()), ok(eq(&8)));
}

#[derive(Debug, PartialEq, Eq)]
enum RequestType {
  Get,
  Head,
}

#[derive(Debug, PartialEq, Eq)]
struct Req {
  req_type: RequestType,
  uri: String,
}

impl Req {
  pub fn new(req_type: RequestType, uri: String) -> Self {
    Self { req_type, uri }
  }
}

parser_generator_impl::grammar! {
  name: GetReq;
  terminal: char;

  <root>: Req => <req> ' ' <uri> {
    Req::new(#req, #uri)
  };

  <req>: RequestType =>
    'G' 'E' 'T' { RequestType::Get } |
    'H' 'E' 'A' 'D' { RequestType::Head };
  <uri>: String => <absoluteURI> {
    #0
  };

  <absoluteURI>: String => ':' <alphas> {
    #0.to_string() + &#alphas
  };

  <alphas>: String => <alphas> <alpha> {
    #alphas + &#alpha.to_string()
  } | <alpha> {
    #alpha.to_string()
  };
  <alpha>: char =>
      'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j'
    | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't'
    | 'u' | 'v' | 'w' | 'x' | 'y' | 'z'
    | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J'
    | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T'
    | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
}

#[gtest]
fn test_parse_uri() {
  expect_that!(
    GetReq::parse("GET :abcdefgh".chars()),
    ok(eq(&Req::new(RequestType::Get, ":abcdefgh".to_string())))
  );
}

#[derive(Debug, PartialEq, Eq)]
enum TestNonLALRPath {
  C1,
  C2,
  C3,
  C4,
}

grammar! {
  name: TestNonLALR;
  terminal: char;

  <T>: TestNonLALRPath => <S> { #S };
  <S>: TestNonLALRPath => <A> 'a' { TestNonLALRPath::C1 };
  <S>: TestNonLALRPath => 'b' <A> 'c' { TestNonLALRPath::C2 };
  <S>: TestNonLALRPath => <B> 'c' { TestNonLALRPath::C3 };
  <S>: TestNonLALRPath => 'b' <B> 'a' { TestNonLALRPath::C4 };
  <A> => 'd';
  <B> => 'd';
}

#[gtest]
fn test_non_lalr() {
  expect_that!(
    TestNonLALR::parse("da".chars()),
    ok(eq(&TestNonLALRPath::C1))
  );
  expect_that!(
    TestNonLALR::parse("bdc".chars()),
    ok(eq(&TestNonLALRPath::C2))
  );
  expect_that!(
    TestNonLALR::parse("dc".chars()),
    ok(eq(&TestNonLALRPath::C3))
  );
  expect_that!(
    TestNonLALR::parse("bda".chars()),
    ok(eq(&TestNonLALRPath::C4))
  );
}
