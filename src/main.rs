#![allow(dead_code)]

use parser_generator::parser::Parser;

struct Test;
impl ::parser_generator::parser::Parser for Test {
  type Token = char;
  type Value = char;
  fn parse<I, B>(input_stream: I) -> ::parser_generator::error::ParserResult<Self::Value>
  where
    I: IntoIterator<Item = B>,
    B: ::std::borrow::Borrow<char>,
  {
    enum TestDfaStates {
      S0(),
      S1(char),
      S2(char),
      S3(char),
      S4(char),
      S5(char),
      S6(char),
      S7(char),
      S8(char),
      S9(char),
      S10(char),
      S11(char),
      S12(char),
      S13(char),
      S14(char),
      S15(char),
      S16(char),
      S17(char),
      S18(char),
      S19(char),
      S20(char),
      S21(char),
      S22(char),
      S23(char),
      S24(char),
      S25(char),
      S26(char),
      S27(char),
      S28(char),
      S29(char),
      S30(char),
      S31(char),
      S32(char),
      S33(char),
      S34(char),
      S35(char),
      S36(char),
      S37(char),
      S38(char),
      S39(char),
      S40(char),
      S41(char),
      S42(char),
      S43(char),
      S44(char),
      S45(char),
      S46(char),
      S47(char),
      S48(char),
      S49(char),
      S50(char),
      S51(char),
      S52(char),
      S53(char),
    }
    fn parse_s0<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        Some(&'a') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S2('a'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'b') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S3('b'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'c') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S4('c'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'d') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S5('d'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'e') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S6('e'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'f') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S7('f'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'g') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S8('g'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'h') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S9('h'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'i') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S10('i'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'j') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S11('j'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'k') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S12('k'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'l') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S13('l'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'m') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S14('m'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'n') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S15('n'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'o') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S16('o'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'p') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S17('p'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'q') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S18('q'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'r') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S19('r'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'s') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S20('s'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'t') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S21('t'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'u') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S22('u'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'v') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S23('v'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'w') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S24('w'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'x') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S25('x'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'y') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S26('y'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'z') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S27('z'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'A') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S28('A'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'B') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S29('B'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'C') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S30('C'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'D') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S31('D'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'E') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S32('E'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'F') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S33('F'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'G') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S34('G'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'H') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S35('H'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'I') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S36('I'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'J') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S37('J'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'K') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S38('K'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'L') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S39('L'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'M') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S40('M'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'N') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S41('N'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'O') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S42('O'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'P') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S43('P'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'Q') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S44('Q'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'R') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S45('R'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'S') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S46('S'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'T') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S47('T'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'U') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S48('U'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'V') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S49('V'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'W') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S50('W'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'X') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S51('X'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'Y') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S52('Y'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        Some(&'Z') => {
          state.stream_mut().advance();
          state.push(TestDfaStates::S53('Z'));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s1<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let TestDfaStates::S1(result) = state.accept() else {
            unsafe { ::std::hint::unreachable_unchecked() }
          };
          Ok(::parser_generator::parser_state::ParserControl::Accept(
            result,
          ))
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s2<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S2(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s3<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S3(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s4<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S4(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s5<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S5(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s6<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S6(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s7<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S7(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s8<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S8(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s9<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S9(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s10<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S10(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s11<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S11(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s12<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S12(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s13<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S13(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s14<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S14(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s15<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S15(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s16<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S16(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s17<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S17(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s18<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S18(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s19<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S19(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s20<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S20(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s21<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S21(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s22<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S22(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s23<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S23(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s24<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S24(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s25<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S25(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s26<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S26(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s27<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S27(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s28<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S28(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s29<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S29(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s30<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S30(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s31<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S31(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s32<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S32(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s33<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S33(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s34<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S34(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s35<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S35(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s36<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S36(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s37<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S37(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s38<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S38(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s39<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S39(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s40<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S40(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s41<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S41(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s42<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S42(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s43<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S43(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s44<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S44(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s45<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S45(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s46<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S46(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s47<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S47(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s48<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S48(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s49<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S49(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s50<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S50(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s51<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S51(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s52<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S52(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    fn parse_s53<I, B: ::std::borrow::Borrow<char>>(
      state: &mut ::parser_generator::parser_state::ParserState<B, TestDfaStates, I>,
    ) -> ::parser_generator::error::ParserResult<
      ::parser_generator::parser_state::ParserControl<char>,
    >
    where
      I: Iterator<Item = B>,
    {
      match state
        .stream()
        .peek_next()
        .map(::std::borrow::Borrow::borrow)
      {
        None => {
          let __v0 = match state.pop_state() {
            TestDfaStates::S53(v) => v,
            _ => unsafe { ::std::hint::unreachable_unchecked() },
          };
          state.push(TestDfaStates::S1({ __v0 }));
          Ok(::parser_generator::parser_state::ParserControl::Continue)
        }
        _ => Err(::parser_generator::error::ParserError::new(
          "Failed to parse",
        )),
      }
    }
    let mut state = ::parser_generator::parser_state::ParserState::new(
      input_stream.into_iter(),
      TestDfaStates::S0(),
    );
    loop {
      let action = match state.state() {
        TestDfaStates::S0(..) => parse_s0(&mut state),
        TestDfaStates::S1(..) => parse_s1(&mut state),
        TestDfaStates::S2(..) => parse_s2(&mut state),
        TestDfaStates::S3(..) => parse_s3(&mut state),
        TestDfaStates::S4(..) => parse_s4(&mut state),
        TestDfaStates::S5(..) => parse_s5(&mut state),
        TestDfaStates::S6(..) => parse_s6(&mut state),
        TestDfaStates::S7(..) => parse_s7(&mut state),
        TestDfaStates::S8(..) => parse_s8(&mut state),
        TestDfaStates::S9(..) => parse_s9(&mut state),
        TestDfaStates::S10(..) => parse_s10(&mut state),
        TestDfaStates::S11(..) => parse_s11(&mut state),
        TestDfaStates::S12(..) => parse_s12(&mut state),
        TestDfaStates::S13(..) => parse_s13(&mut state),
        TestDfaStates::S14(..) => parse_s14(&mut state),
        TestDfaStates::S15(..) => parse_s15(&mut state),
        TestDfaStates::S16(..) => parse_s16(&mut state),
        TestDfaStates::S17(..) => parse_s17(&mut state),
        TestDfaStates::S18(..) => parse_s18(&mut state),
        TestDfaStates::S19(..) => parse_s19(&mut state),
        TestDfaStates::S20(..) => parse_s20(&mut state),
        TestDfaStates::S21(..) => parse_s21(&mut state),
        TestDfaStates::S22(..) => parse_s22(&mut state),
        TestDfaStates::S23(..) => parse_s23(&mut state),
        TestDfaStates::S24(..) => parse_s24(&mut state),
        TestDfaStates::S25(..) => parse_s25(&mut state),
        TestDfaStates::S26(..) => parse_s26(&mut state),
        TestDfaStates::S27(..) => parse_s27(&mut state),
        TestDfaStates::S28(..) => parse_s28(&mut state),
        TestDfaStates::S29(..) => parse_s29(&mut state),
        TestDfaStates::S30(..) => parse_s30(&mut state),
        TestDfaStates::S31(..) => parse_s31(&mut state),
        TestDfaStates::S32(..) => parse_s32(&mut state),
        TestDfaStates::S33(..) => parse_s33(&mut state),
        TestDfaStates::S34(..) => parse_s34(&mut state),
        TestDfaStates::S35(..) => parse_s35(&mut state),
        TestDfaStates::S36(..) => parse_s36(&mut state),
        TestDfaStates::S37(..) => parse_s37(&mut state),
        TestDfaStates::S38(..) => parse_s38(&mut state),
        TestDfaStates::S39(..) => parse_s39(&mut state),
        TestDfaStates::S40(..) => parse_s40(&mut state),
        TestDfaStates::S41(..) => parse_s41(&mut state),
        TestDfaStates::S42(..) => parse_s42(&mut state),
        TestDfaStates::S43(..) => parse_s43(&mut state),
        TestDfaStates::S44(..) => parse_s44(&mut state),
        TestDfaStates::S45(..) => parse_s45(&mut state),
        TestDfaStates::S46(..) => parse_s46(&mut state),
        TestDfaStates::S47(..) => parse_s47(&mut state),
        TestDfaStates::S48(..) => parse_s48(&mut state),
        TestDfaStates::S49(..) => parse_s49(&mut state),
        TestDfaStates::S50(..) => parse_s50(&mut state),
        TestDfaStates::S51(..) => parse_s51(&mut state),
        TestDfaStates::S52(..) => parse_s52(&mut state),
        TestDfaStates::S53(..) => parse_s53(&mut state),
      }?;
      match action {
        ::parser_generator::parser_state::ParserControl::Accept(result) => {
          return Ok(result);
        }
        ::parser_generator::parser_state::ParserControl::Continue => {}
      }
    }
  }
}

fn main() {
  let res = Test::parse(['a']);
  match res {
    Ok(res) => println!("{res}"),
    Err(err) => eprintln!("{err}"),
  }

  // parser_generator::grammar!(
  //   name: Test;
  //   terminal: char;

  //   <S>: char => <dig> { #dig };
  //   <dig>: char =>
  //         'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' |
  //         'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' |
  //         'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
  //         'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' |
  //         'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' |
  //         'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
  // );

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
