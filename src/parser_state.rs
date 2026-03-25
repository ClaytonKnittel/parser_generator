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

  pub fn stream_mut(&mut self) -> &mut ParserStream<T, I> {
    &mut self.stream
  }

  pub fn push(&mut self, state: S) {
    self.stack.push(state);
  }

  pub fn state(&self) -> &S {
    self.stack.last().unwrap()
  }

  pub fn pop_state(&mut self) -> S {
    self.stack.pop().unwrap()
  }

  pub fn accept(&mut self) -> S {
    // In the accept state, the stack always consists of
    // [initial_state, accepted_state(accept_val)]
    debug_assert_eq!(self.stack.len(), 2);
    self.stack.pop().unwrap()
  }
}
