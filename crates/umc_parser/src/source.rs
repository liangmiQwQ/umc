use std::str::Chars;

pub struct Source<'a> {
  pub pointer: u32,
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
    self.source_text[(self.pointer as usize)..].chars()
  }

  /// Unsafe, panic expected if bytes wrong
  pub fn advance_bytes(&mut self, bytes: u32) {
    let target = self.pointer + bytes;
    self.pointer = target;
  }
}
