use googletest::prelude::*;
use parser_generator::{grammar, parser::Parser};

#[derive(Clone)]
enum Keyword {
  Public,
  Static,
  Void,
}

#[derive(Clone)]
struct Ident {
  name: String,
  spacing: Spacing,
}

#[derive(Clone)]
struct Literal(String);

#[derive(Clone)]
enum Op {
  Eq,
  Semicolon,
}

#[derive(Clone)]
enum Spacing {
  Alone,
  Joint,
}

#[derive(Clone)]
struct Operator {
  op: Op,
  spacing: Spacing,
}

#[derive(Clone)]
enum Token {
  Keyword(Keyword),
  Ident(Ident),
  Literal(Literal),
  Operator(Operator),
}

#[derive(Clone, Debug)]
struct MainMethod {
  name: String,
  value: String,
}

grammar! {
  name: TokenPattern;
  enum_terminal: Token;

  <root>: MainMethod =>
    Keyword(Keyword::Public) Keyword(Keyword::Static) Keyword(Keyword::Void)
    <ident> <eq> <literal> <semicolon> {
    MainMethod { name: #ident.0, value: #literal.0 }
  };
  <ident>: Ident => Ident(Ident { spacing: Spacing::Alone, .. });
  <eq> =>
    Operator(Operator { op: Op::Eq, spacing: Spacing::Joint })
    Operator(Operator { op: Op::Eq, spacing: Spacing::Alone });
  <semicolon> =>
    Operator(Operator { op: Op::Semicolon, spacing: Spacing::Alone });
  <literal>: Literal => Literal(..) {
    #0
  };
}

#[gtest]
fn test_parse() {
  let result: MainMethod = TokenPattern::parse([
    Token::Keyword(Keyword::Public),
    Token::Keyword(Keyword::Static),
    Token::Keyword(Keyword::Void),
    Token::Ident(Ident {
      name: "main".to_string(),
      spacing: Spacing::Alone,
    }),
    Token::Operator(Operator {
      op: Op::Eq,
      spacing: Spacing::Joint,
    }),
    Token::Operator(Operator {
      op: Op::Eq,
      spacing: Spacing::Alone,
    }),
    Token::Literal(Literal("123".to_string())),
    Token::Operator(Operator {
      op: Op::Semicolon,
      spacing: Spacing::Alone,
    }),
  ])
  .unwrap();

  expect_that!(
    result,
    pat![MainMethod {
      name: eq("main"),
      value: eq("123")
    }]
  );
}

#[gtest]
fn test_parse_fail() {
  let result = TokenPattern::parse([
    Token::Keyword(Keyword::Static),
    Token::Keyword(Keyword::Public),
    Token::Keyword(Keyword::Void),
    Token::Ident(Ident {
      name: "main".to_string(),
      spacing: Spacing::Alone,
    }),
    Token::Operator(Operator {
      op: Op::Eq,
      spacing: Spacing::Alone,
    }),
    Token::Literal(Literal("123".to_string())),
  ]);

  expect_that!(result, err(anything()));
}
