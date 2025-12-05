use std::str::Chars;

pub struct Source<'a> {
  pub pointer: usize,
  pub source_text: &'a str,
}

impl<'a> Source<'a> {
  pub fn new(source_text: &'a str) -> Source<'a> {
    Source {
      pointer: 0,
      source_text,
    }
  }
}

impl<'a> Source<'a> {
  pub fn get_chars(&self) -> Chars<'a> {
    self.source_text[self.pointer..].chars()
  }

  /// Unsafe, panic expected if bytes wrong
  pub fn advance_bytes(&mut self, bytes: usize) {
    let target = self.pointer + bytes;
    self.pointer = target;
  }
}
