use googletest::{
  expect_that, gtest,
  prelude::{anything, none, some},
};

#[gtest]
fn test_large_or() -> googletest::Result<()> {
  parser_generator::grammar! {
    name: Test;
    terminal: char;

    <S>: char => <dig> { #dig };
    <dig>: char =>
          'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' |
          'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' |
          'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
          'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' |
          'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' |
          'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
  };

  for letter in ('a'..='z').chain('A'..='Z') {
    expect_that!(
      Test::parse([letter].into_iter().peekable()),
      some(anything())
    );
  }
  expect_that!(Test::parse(".".chars().into_iter().peekable()), none());

  Ok(())
}
