use std::ops::Range;

use super::kind::Html5Kind;

#[derive(Debug, PartialEq)]
pub struct Html5Token {
  pub kind: Html5Kind,
  pub start: usize,
  pub end: usize,
}

impl Html5Token {
  pub fn range(&self) -> Range<usize> {
    self.start..self.end
  }
}
