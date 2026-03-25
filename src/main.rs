#![allow(dead_code)]

fn main() {
  parser_generator::grammar!(
    name: Test;
    terminal: char;

    <S>: u32 => <A>;
    <A>: u32 => <B>;
    <A>: u32 => 'a' <C>;
    <B>: u32 => 'x' 'b';
    <B>: u32 => <C>;
    <C>: u32 => 'x' 'c';
  );

  /*
  parser_generator_impl::grammar_def! {
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
          '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
        | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j'
        | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't'
        | 'u' | 'v' | 'w' | 'x' | 'y' | 'z'
        | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J'
        | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T'
        | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
  };

  let res = Test::parse("21*42+1000".chars().into_iter().peekable());

  match res {
    Some((i, _)) => println!("Result: {}", i),
    None => println!("no match :("),
  }
  */

  // struct Test {}
  // impl Test {
  //   /// Parses an input stream according to the grammar, returning the
  //   /// constructed object from a correctly formatted input, or None if the
  //   /// input was not a sentential form of the grammar.
  //   ///
  //   /// This variant of parse uses an iterator over references to the
  //   /// terminal type.
  //   pub fn parse_ref<'a, I: Iterator<Item = &'a char>>(
  //     mut input_stream: std::iter::Peekable<I>,
  //   ) -> Option<(u32, std::iter::Peekable<I>)> {
  //     #[derive(Debug)]
  //     enum TestDfaStates {
  //       S10(char),
  //       S6(char),
  //       S5(char),
  //       S9(char),
  //       S2(u32),
  //       S4(u32),
  //       S3(u32),
  //       S0(u32),
  //       S8,
  //       S7(char),
  //       S1(u32),
  //       T(u32),
  //     }

  //     let mut states = vec![TestDfaStates::S8];
  //     loop {
  //       let state = states.last().unwrap();
  //       let next_token = input_stream.peek();
  //       match (state, next_token) {
  //         (TestDfaStates::S10(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(*_term_val));
  //         }
  //         (TestDfaStates::S6(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(*_term_val));
  //         }
  //         (TestDfaStates::S5(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S5(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0.to_digit(10).unwrap() };
  //           match states.last() {
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             Some(TestDfaStates::S10(_)) => {
  //               states.push(TestDfaStates::S4(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S9(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S9(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           match states.last() {
  //             Some(TestDfaStates::S4(_)) => {
  //               states.push(TestDfaStates::S7(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S5(cons));
  //             }
  //             Some(TestDfaStates::S2(_)) => {
  //               states.push(TestDfaStates::S7(cons));
  //             }
  //             Some(TestDfaStates::S10(_)) => {
  //               states.push(TestDfaStates::S5(cons));
  //             }
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S5(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S2(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(*_term_val));
  //         }
  //         (TestDfaStates::S2(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S2(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           match states.last() {
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S0(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S1(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S4(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(*_term_val));
  //         }
  //         (TestDfaStates::S4(_), _) => {
  //           let __parser_generator_impl_v2 = match states.pop() {
  //             Some(TestDfaStates::S4(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v1 = match states.pop() {
  //             Some(TestDfaStates::S10(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S0(val)) => val,
  //             Some(TestDfaStates::S1(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 * __parser_generator_impl_v2 };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S1(cons));
  //             }
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S0(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S3(_), Some(&_term_val @ '+')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S6(*_term_val));
  //         }
  //         (TestDfaStates::S3(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S3(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           return Some((cons, input_stream));
  //         }
  //         (TestDfaStates::S0(_), Some(&_term_val @ '*')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S10(*_term_val));
  //         }
  //         (TestDfaStates::S0(_), _) => {
  //           let __parser_generator_impl_v2 = match states.pop() {
  //             Some(TestDfaStates::S0(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v1 = match states.pop() {
  //             Some(TestDfaStates::S6(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S3(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 + __parser_generator_impl_v2 };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S3(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S8, Some(&_term_val @ '0'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '1'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '2'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '3'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '4'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '5'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '6'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '7'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '8'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '9'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 's'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 't'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(*_term_val));
  //         }
  //         (TestDfaStates::S7(_), _) => {
  //           let __parser_generator_impl_v1 = match states.pop() {
  //             Some(TestDfaStates::S7(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S2(val)) => val,
  //             Some(TestDfaStates::S4(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = {
  //             10 * __parser_generator_impl_v0 + __parser_generator_impl_v1.to_digit(10).unwrap()
  //           };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             Some(TestDfaStates::S10(_)) => {
  //               states.push(TestDfaStates::S4(cons));
  //             }
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S1(_), Some(&_term_val @ '*')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S10(*_term_val));
  //         }
  //         (TestDfaStates::S1(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S1(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S3(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         _ => {
  //           match next_token {
  //             Some(token) => {
  //               {
  //                 eprint!("Unexpected token \"{0}\"\n", token);
  //               };
  //             }
  //             None => {
  //               {
  //                 eprint!("Unexpected end of input\n");
  //               };
  //             }
  //           }
  //           return None;
  //         }
  //       }
  //     }
  //   }
  //   /// Parses an input stream according to the grammar, returning the
  //   /// constructed object from a correctly formatted input, or None if the
  //   /// input was not a sentential form of the grammar.
  //   pub fn parse<I: Iterator<Item = char>>(
  //     mut input_stream: std::iter::Peekable<I>,
  //   ) -> Option<(u32, std::iter::Peekable<I>)> {
  //     #[derive(Debug)]
  //     enum TestDfaStates {
  //       S10(char),
  //       S6(char),
  //       S5(char),
  //       S9(char),
  //       S2(u32),
  //       S4(u32),
  //       S3(u32),
  //       S0(u32),
  //       S8,
  //       S7(char),
  //       S1(u32),
  //       T(u32),
  //     }

  //     let mut states = vec![TestDfaStates::S8];
  //     loop {
  //       let state = states.last().unwrap();
  //       let next_token = input_stream.peek();
  //       match (state, next_token) {
  //         (TestDfaStates::S10(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S10(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(_term_val));
  //         }
  //         (TestDfaStates::S6(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S6(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(_term_val));
  //         }
  //         (TestDfaStates::S5(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S5(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0.to_digit(10).unwrap() };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             Some(TestDfaStates::S10(_)) => {
  //               states.push(TestDfaStates::S4(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S9(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S9(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           match states.last() {
  //             Some(TestDfaStates::S2(_)) => {
  //               states.push(TestDfaStates::S7(cons));
  //             }
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S5(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S5(cons));
  //             }
  //             Some(TestDfaStates::S4(_)) => {
  //               states.push(TestDfaStates::S7(cons));
  //             }
  //             Some(TestDfaStates::S10(_)) => {
  //               states.push(TestDfaStates::S5(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S2(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S2(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(_term_val));
  //         }
  //         (TestDfaStates::S2(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S2(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           match states.last() {
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S0(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S1(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S4(_), Some(&_term_val @ '0'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '1'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '2'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '3'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '4'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '5'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '6'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '7'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '8'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ '9'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 's'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 't'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S4(_), Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(_term_val));
  //         }
  //         (TestDfaStates::S4(_), _) => {
  //           let __parser_generator_impl_v2 = match states.pop() {
  //             Some(TestDfaStates::S4(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v1 = match states.pop() {
  //             Some(TestDfaStates::S10(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S1(val)) => val,
  //             Some(TestDfaStates::S0(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 * __parser_generator_impl_v2 };
  //           match states.last() {
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S0(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S1(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S3(_), Some(&_term_val @ '+')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S6(_term_val));
  //         }
  //         (TestDfaStates::S3(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S3(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           return Some((cons, input_stream));
  //         }
  //         (TestDfaStates::S0(_), Some(&_term_val @ '*')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S10(_term_val));
  //         }
  //         (TestDfaStates::S0(_), _) => {
  //           let __parser_generator_impl_v2 = match states.pop() {
  //             Some(TestDfaStates::S0(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v1 = match states.pop() {
  //             Some(TestDfaStates::S6(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S3(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 + __parser_generator_impl_v2 };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S3(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S8, Some(&_term_val @ '0'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '1'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '2'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '3'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '4'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '5'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '6'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '7'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '8'))
  //         | (TestDfaStates::S8, Some(&_term_val @ '9'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'A'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'B'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'C'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'D'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'E'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'F'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'G'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'H'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'I'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'J'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'K'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'L'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'M'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'N'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'O'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'P'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'Q'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'R'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'S'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'T'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'U'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'V'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'W'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'X'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'Y'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'Z'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'a'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'b'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'c'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'd'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'e'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'f'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'g'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'h'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'i'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'j'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'k'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'l'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'm'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'n'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'o'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'p'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'q'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'r'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 's'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 't'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'u'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'v'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'w'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'x'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'y'))
  //         | (TestDfaStates::S8, Some(&_term_val @ 'z')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S9(_term_val));
  //         }
  //         (TestDfaStates::S7(_), _) => {
  //           let __parser_generator_impl_v1 = match states.pop() {
  //             Some(TestDfaStates::S7(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S2(val)) => val,
  //             Some(TestDfaStates::S4(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = {
  //             10 * __parser_generator_impl_v0 + __parser_generator_impl_v1.to_digit(10).unwrap()
  //           };
  //           match states.last() {
  //             Some(TestDfaStates::S6(_)) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S2(cons));
  //             }
  //             Some(TestDfaStates::S10(_)) => {
  //               states.push(TestDfaStates::S4(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         (TestDfaStates::S1(_), Some(&_term_val @ '*')) => {
  //           input_stream.next();
  //           states.push(TestDfaStates::S10(_term_val));
  //         }
  //         (TestDfaStates::S1(_), _) => {
  //           let __parser_generator_impl_v0 = match states.pop() {
  //             Some(TestDfaStates::S1(val)) => val,
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           };
  //           let cons = { __parser_generator_impl_v0 };
  //           match states.last() {
  //             Some(TestDfaStates::S8) => {
  //               states.push(TestDfaStates::S3(cons));
  //             }
  //             _ => unsafe { std::hint::unreachable_unchecked() },
  //           }
  //         }
  //         _ => {
  //           match next_token {
  //             Some(token) => {
  //               {
  //                 eprint!("Unexpected token \"{0}\"\n", token);
  //               };
  //             }
  //             None => {
  //               {
  //                 eprint!("Unexpected end of input\n");
  //               };
  //             }
  //           }
  //           return None;
  //         }
  //       }
  //     }
  //   }
  // }
  // let res = Test::parse("21*42+1000".chars().into_iter().peekable());
  // match res {
  //   Some((i, _)) => {
  //     print!("Result: {0}\n", i);
  //   }
  //   None => {
  //     print!("no match :(\n");
  //   }
  // }

  /*
  parser_generator_impl::grammar_def! {
    terminal: CToken;

    <program> => <declList>;
    <declList> => <declList> <decl> | <decl>;
    <decl> => <varDecl> | <funDecl>;
    <varDecl> => <typeSpec> <varDeclList> ';';
    <scopedVarDecl> => <static> <typeSpec> <varDeclList> ';' | <typeSpec> <varDeclList> ';';
    <varDeclList> => <varDeclList> ',' <varDeclInit> | <varDeclInit>;
    <varDeclInit> => <varDeclId> | <varDeclId> ':' <simpleExp>;
    <varDeclId> => <ID> | <ID> '[' <NUMCONST> ']';
    <typeSpec> => <int> | <bool> | <char>;
    <funDecl> => <typeSpec> <ID> '(' <parms> ')' <stmt> | <ID> '(' <parms> ')' <stmt>;
    <parms> => <parmList> | !;
    <parmList> => <parmList> ';' <parmTypeList> | <parmTypeList>;
    <parmTypeList> => <typeSpec> <parmIdList>;
    <parmIdList> => <parmIdList> ',' <parmId> | <parmId>;
    <parmId>=><ID> | <ID> '[' ']';
    <stmt> => <expStmt> | <compoundStmt> | <selectStmt> | <iterStmt> | <returnStmt> | <breakStmt>;
    <expStmt> => <exp> ';' | ';';
    <compoundStmt> => '{' <localDecls> <stmtList> '}';
    <localDecls> => <localDecls> <scopedVarDecl> | !;
    <stmtList> => <stmtList> <stmt> | !;
    <selectStmt> => <if> <simpleExp> <then> <stmt> | <if> <simpleExp> <then> <stmt> <else> <stmt>;
    <iterStmt> => <while> <simpleExp> <do> <stmt> | <for> <ID> '=' <iterRange> <do> <stmt>;
    <iterRange> => <simpleExp> <to> <simpleExp> | <simpleExp> <to> <simpleExp> <by> <simpleExp>;
    <returnStmt> => <return> ';' | <return> <exp> ';';
    <breakStmt> => <break> ';';
    <exp> => <mutable> '=' <exp>
           | <mutable> '+' '=' <exp>
           | <mutable> '-' '=' <exp>
           | <mutable> '*' '=' <exp>
           | <mutable> '/' '=' <exp>
           | <mutable> '+' '+'
           | <mutable> '-' '-'
           | <simpleExp>;
    <simpleExp> => <simpleExp> <or> <andExp> | <andExp>;
    <andExp> => <andExp> <and> <unaryRelExp> | <unaryRelExp>;
    <unaryRelExp> => <not> <unaryRelExp> | <relExp>;
    <relExp> => <minmaxExp> <relop> <minmaxExp> | <minmaxExp>;
    <relop> => '<' '=' | '<' | '>' | '>' '=' | 'q' | '!' '=';
    <minmaxExp> => <minmaxExp> <minmaxop> <sumExp> | <sumExp>;
    <minmaxop> => ':' '>' ':' | ':' '<' ':';
    <sumExp> => <sumExp> <sumop> <mulExp> | <mulExp>;
    <sumop>=> '+' | '-';
    <mulExp> => <mulExp> <mulop> <unaryExp> | <unaryExp>;
    <mulop> => '*' | '/' | '%';
    <unaryExp> => <unaryop> <unaryExp> | <factor>;
    <unaryop> => '-' | '*' | '?';
    <factor> => <immutable> | <mutable>;
    <mutable> => <ID> | <ID> '[' <exp> ']';
    <immutable> => '(' <exp> ')' | <call> | <constant>;
    <call>=> <ID> '(' <args> ')';
    <args> => <argList> | !;
    <argList> => <argList> ',' <exp> | <exp>;
    <constant> => <NUMCONST> | <CHARCONST> | <STRINGCONST> | <true> | <false>;

    <return> => 'a';
    <or> => '|';
    <and> => '&';
    <not> => '!';
    <ID> => 'x';
    <NUMCONST> => '1';
    <CHARCONST> => '\'';
    <if> => 'i';
    <else> => 'e';
    <for> => 'f';
    <while> => 'w';
    <do> => 'd';
    <break> => 'r';
    <then> => 't';
    <to> => '2';
    <by> => 'b';
    <static> => 's';
    <int> => '0';
    <bool> => '3';
    <char> => '4';
    <STRINGCONST> => '5';
    <true> => 'y';
    <false> => 'z';
  }
  */
}

// Regexes:
// ::std::io::_(e?)print\([\s\r]*format_args!\([\s\r]*(.+)[\s\r]*\)[\s\r]*\);
// $1print!($2);
// ::core::panicking::panic\([\s\r]*"internal error: entered unreachable code",?[\s\r]*\)
// unreachable!()
// <\[_\]>::into_vec\([\s\r]*#\[rustc_box\][\s\r]*::alloc::boxed::Box::new\((\[[^\]]+\])\),[\s\r]*\)
// vec!$1
