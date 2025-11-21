use std::ops::Range;

use super::kind::Html5Kind;

#[derive(Debug, Default)]
pub struct Html5Token {
  pub kind: Html5Kind,
  pub start: usize,
  pub end: usize,
  pub value: Html5TokenValue,
}

impl Html5Token {
  #[must_use]
  pub const fn range(&self) -> Range<usize> {
    self.start..self.end
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Html5TokenValue {
  #[default]
  None,
  Number(f64),
  String(String),
}
