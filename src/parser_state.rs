use crate::parser_stream::ParserStream;

pub enum ParserControl<T> {
  Continue,
  Accept(T),
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

  pub fn stream(&self) -> &ParserStream<T, I> {
    &self.stream
  }

  pub fn push(&mut self, state: S) {
    self.stack.push(state);
  }

  pub fn state(&self) -> &S {
    self.stack.last().unwrap()
  }

  pub fn accept(&mut self) -> S {
    debug_assert_eq!(self.stack.len(), 1);
    self.stack.pop().unwrap()
  }
}
