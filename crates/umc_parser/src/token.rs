use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct Token<T> {
  pub kind: T,
  pub start: usize,
  pub end: usize,
}

impl<T> Token<T> {
  pub fn range(&self) -> Range<usize> {
    self.start..self.end
  }
}
