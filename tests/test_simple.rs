use googletest::prelude::*;
use parser_generator::parser::Parser;

#[gtest]
fn test_simple() -> googletest::Result<()> {
  parser_generator::pub_grammar! {
    name: Test;
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
    <V>: u32 => <V> <dig> {
      10 * #V + #dig.to_digit(10).unwrap()
    } | <dig> {
      #dig.to_digit(10).unwrap()
    };
    <dig>: char =>
          '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
  }

  let res = Test::parse("21*42+1000".chars().peekable())?;

  expect_eq!(res, 1882);

  Ok(())
}
