use std::{
  error::Error,
  fmt::{Debug, Display},
};

use cknittel_util::iter::CollectResult;
use googletest::prelude::*;
use parser_generator::{grammar, parser::ParserNoContext};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
  IncTape,
  DecTape,
  IncByte,
  DecByte,
  Output,
  Input,
  OpenBracket,
  CloseBracket,
}

impl Instruction {
  fn from_char(c: char) -> Self {
    match c {
      '>' => Self::IncTape,
      '<' => Self::DecTape,
      '+' => Self::IncByte,
      '-' => Self::DecByte,
      '.' => Self::Output,
      ',' => Self::Input,
      '[' => Self::OpenBracket,
      ']' => Self::CloseBracket,
      _ => panic!("Unrecognized character: \"{c}\""),
    }
  }
}

#[derive(Clone, Copy, Default)]
struct InstructionPointer(usize);

impl InstructionPointer {
  fn inc(&mut self) {
    self.0 += 1;
  }

  fn dec(&mut self) {
    self.0 -= 1;
  }

  fn prev(&self) -> Self {
    Self(self.0 - 1)
  }
}

struct InstructionList {
  instructions: Vec<Instruction>,
}

impl InstructionList {
  fn with(mut self, instruction: Instruction) -> Self {
    self.push(instruction);
    self
  }

  fn push(&mut self, instruction: Instruction) {
    self.instructions.push(instruction);
  }

  fn extend(&mut self, instructions: impl IntoIterator<Item = Instruction>) {
    self.instructions.extend(instructions);
  }

  fn get(&self, pc: InstructionPointer) -> Option<&Instruction> {
    if pc.0 < self.instructions.len() {
      Some(&self.instructions[pc.0])
    } else {
      None
    }
  }
}

impl IntoIterator for InstructionList {
  type Item = Instruction;
  type IntoIter = <Vec<Instruction> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.instructions.into_iter()
  }
}

grammar! {
  name: BrainFck;
  enum_terminal: Instruction;

  <root>: InstructionList => <instruction_list>;
  <instruction_list>: InstructionList =>
    ! { InstructionList { instructions: Vec::new() } } |
    <instruction_list> <instruction> { #instruction_list.with(#instruction) } |
    <instruction_list> OpenBracket <instruction_list> CloseBracket {
      #0.push(Instruction::OpenBracket);
      #0.extend(#2);
      #0.push(Instruction::CloseBracket);
      #0
    };
  <instruction>: Instruction => IncTape;
  <instruction>: Instruction => DecTape;
  <instruction>: Instruction => IncByte;
  <instruction>: Instruction => DecByte;
  <instruction>: Instruction => Output;
  <instruction>: Instruction => Input;
}

enum BrainFckError {
  EndOfInput,
  ProgramCounterBelowZero,
  Halt,
}
impl Display for BrainFckError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::EndOfInput => "End of input",
        Self::ProgramCounterBelowZero => "The program counter was decremented below 0",
        Self::Halt => "The program has finished",
      }
    )
  }
}
impl Debug for BrainFckError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
impl Error for BrainFckError {}

type BrainFckResult<T = ()> = ::core::result::Result<T, BrainFckError>;

struct BrainFckState<'a, I> {
  instructions: &'a InstructionList,
  pc: InstructionPointer,
  tape: Vec<u8>,
  position: usize,
  input: I,
}

impl<'a, I> BrainFckState<'a, I> {
  fn new(instructions: &'a InstructionList, input: I) -> Self {
    Self {
      instructions,
      pc: InstructionPointer::default(),
      tape: vec![0],
      position: 0,
      input,
    }
  }
}

impl<'a, I> BrainFckState<'a, I>
where
  I: Iterator<Item = u8>,
{
  fn advance(&mut self) -> BrainFckResult<Option<u8>> {
    let instruction = self.instructions.get(self.pc).ok_or(BrainFckError::Halt)?;
    self.pc.inc();
    match instruction {
      Instruction::IncTape => {
        self.position += 1;
        if self.position == self.tape.len() {
          self.tape.push(0);
        }
        Ok(None)
      }
      Instruction::DecTape => {
        if self.position == 0 {
          Err(BrainFckError::ProgramCounterBelowZero)
        } else {
          self.position -= 1;
          Ok(None)
        }
      }
      Instruction::IncByte => {
        self.tape[self.position] = self.tape[self.position].wrapping_add(1);
        Ok(None)
      }
      Instruction::DecByte => {
        self.tape[self.position] = self.tape[self.position].wrapping_sub(1);
        Ok(None)
      }
      Instruction::Output => Ok(Some(self.tape[self.position])),
      Instruction::Input => {
        self.tape[self.position] = self.input.next().ok_or(BrainFckError::EndOfInput)?;
        Ok(None)
      }
      Instruction::OpenBracket => {
        if self.tape[self.position] == 0 {
          while self.instructions.get(self.pc).unwrap() != &Instruction::CloseBracket {
            self.pc.inc();
          }
          self.pc.inc();
        }

        Ok(None)
      }
      Instruction::CloseBracket => {
        if self.tape[self.position] != 0 {
          self.pc.dec();
          while self.instructions.get(self.pc.prev()).unwrap() != &Instruction::OpenBracket {
            self.pc.dec();
          }
        }

        Ok(None)
      }
    }
  }
}

impl<'a, I> Iterator for BrainFckState<'a, I>
where
  I: Iterator<Item = u8>,
{
  type Item = BrainFckResult<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.advance() {
        Ok(Some(res)) => return Some(Ok(res)),
        Ok(None) => {}
        Err(BrainFckError::Halt) => return None,
        Err(err) => return Some(Err(err)),
      }
    }
  }
}

pub trait LifetimeBoundIterator<'a, T>: Iterator<Item = T>
where
  Self::Item: 'a,
{
}
impl<'a, I, T> LifetimeBoundIterator<'a, T> for I
where
  I: Iterator<Item = T>,
  T: 'a,
{
}

fn execute<'a>(
  instructions: &'a InstructionList,
  input: impl IntoIterator<Item = u8>,
) -> impl LifetimeBoundIterator<'a, BrainFckResult<u8>> {
  BrainFckState::new(instructions, input.into_iter())
}

#[gtest]
fn count_to_ten() {
  let program = BrainFck::parse(
    "+++++++++++++++++++++++++++++++++++++++++++++++++>\
     ++++++++++>\
     +++++++++[<<.+>.>-]\
     +++++++++[<<->>-]\
     <<.-.>."
      .chars()
      .map(Instruction::from_char),
  )
  .unwrap();

  #[rustfmt::skip]
  expect_that!(
    execute(&program, []).collect_result_vec(),
    ok(elements_are![
      eq(&b'1'), eq(&b'\n'),
      eq(&b'2'), eq(&b'\n'),
      eq(&b'3'), eq(&b'\n'),
      eq(&b'4'), eq(&b'\n'),
      eq(&b'5'), eq(&b'\n'),
      eq(&b'6'), eq(&b'\n'),
      eq(&b'7'), eq(&b'\n'),
      eq(&b'8'), eq(&b'\n'),
      eq(&b'9'), eq(&b'\n'),
      eq(&b'1'), eq(&b'0'), eq(&b'\n'),
    ])
  );
}

#[gtest]
fn add_two() {
  let program = BrainFck::parse(
    ",>,\
     [<+>-]\
     <------------------------------------------------."
      .chars()
      .map(Instruction::from_char),
  )
  .unwrap();

  expect_that!(
    execute(&program, [b'1', b'2']).collect_result_vec(),
    ok(elements_are![eq(&b'3')])
  );
  expect_that!(
    execute(&program, [b'3', b'5']).collect_result_vec(),
    ok(elements_are![eq(&b'8')])
  );
}
