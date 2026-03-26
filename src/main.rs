use parser_generator::parser::Parser;

enum BFC {
  IncTape,
  DecTape,
  IncByte,
  DecByte,
  Output,
  Input,
  OpenBracket,
  CloseBracket,
}

impl BFC {
  fn from_char(c: char) -> Self {
    match c {
      '>' => BFC::IncTape,
      '<' => BFC::DecTape,
      '+' => BFC::IncByte,
      '-' => BFC::DecByte,
      '.' => BFC::Output,
      ',' => BFC::Input,
      '[' => BFC::OpenBracket,
      ']' => BFC::CloseBracket,
      _ => panic!("Unrecognized character: \"{c}\""),
    }
  }
}

parser_generator::grammar! {
  name: BrainFck;
  enum_terminal: BFC;

  <root>: u32 => <instruction_list>;
  <instruction_list>: u32 =>
    <instruction> |
    <instruction_list> <instruction> { #instruction_list + #instruction } |
    <instruction_list> OpenBracket <instruction_list> CloseBracket {
      #0 + 7 + #0 + 8
    };
  <instruction>: u32 => IncTape { 1 };
  <instruction>: u32 => DecTape { 2 };
  <instruction>: u32 => IncByte { 3 };
  <instruction>: u32 => DecByte { 4 };
  <instruction>: u32 => Output { 5 };
  <instruction>: u32 => Input { 6 };
}

fn main() {
  println!(
    "Result: {}",
    BrainFck::parse("++,,[.]".chars().map(BFC::from_char)).unwrap()
  );
}
