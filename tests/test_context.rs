use googletest::prelude::*;
use parser_generator::{grammar, parser::Parser};

#[derive(Default)]
struct ParseContext {
  info: Vec<String>,
}

grammar!(
  name: AutoInto;
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
  let result = AutoInto::parse_with_ctx("a".chars(), &mut ctx);

  expect_that!(result, ok(()));
  expect_that!(ctx.info, elements_are!["updated!"]);
}
