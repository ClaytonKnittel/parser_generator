use crate::parser_stream::ParserStream;

pub enum ParserControl {
  Continue,
  Accept,
}

pub struct ParserState<T, S, I> {
  stream: ParserStream<T, I>,
  stack: Vec<S>,
}

impl<T, S, I> ParserState<T, S, I> {
  pub fn new(iter: I, initial_state: S) -> Self
  where
    I: Iterator<Item = T>,
  {
    Self {
      stream: ParserStream::new(iter),
      stack: vec![initial_state],
    }
  }

  pub fn state(&self) -> &S {
    self.stack.last().unwrap()
  }
}
