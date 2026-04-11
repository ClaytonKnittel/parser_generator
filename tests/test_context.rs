use googletest::prelude::*;
use parser_generator::{
  grammar,
  parser::{Parser, ParserNoContext},
};

#[derive(Default)]
struct ParseContext {
  info: Vec<String>,
}

grammar!(
  name: ContextText;
  terminal: char;
  context_type: ParseContext;

  <root>: () => <a>;

  <a>: () => 'a' {
    #ctx.info.push("updated!".into());
  };
);

#[gtest]
fn test_context() {
  let mut ctx = ParseContext::default();
  let result = ContextText::parse_with_ctx("a".chars(), &mut ctx);

  expect_that!(result, ok(()));
  expect_that!(ctx.info, elements_are!["updated!"]);
}

#[derive(Clone, Debug, PartialEq)]
enum UseOfCtxNameOkResult {
  Ok,
}

grammar!(
  name: UseOfCtxNameOk;
  terminal: char;

  <root>: UseOfCtxNameOkResult => <ctx> { #ctx };

  <ctx>: UseOfCtxNameOkResult => 'a' { UseOfCtxNameOkResult::Ok };
);

#[gtest]
fn test_use_of_ctx_symbol_allowed_if_no_context_type_specified() {
  let result = UseOfCtxNameOk::parse("a".chars());
  expect_that!(result, ok(eq(&UseOfCtxNameOkResult::Ok)));
}
