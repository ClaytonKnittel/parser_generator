pub struct ParserStream<T, I> {
  iter: I,
  next: Option<T>,
}

impl<T, I> ParserStream<T, I>
where
  I: Iterator<Item = T>,
{
  pub fn new(mut iter: I) -> Self {
    let next = iter.next();
    Self { iter, next }
  }

  pub fn peek_next(&self) -> Option<&T> {
    self.next.as_ref()
  }

  pub fn advance(&mut self) {
    debug_assert!(self.next.is_some());
    self.next = self.iter.next();
  }
}
