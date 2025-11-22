use std::str::Chars;

pub(crate) struct Source<'a> {
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

  pub fn current(&self) -> Option<char> {
    self.get_chars().next()
  }

  pub fn advance_chars(&mut self, chars: usize) {
    let mut diff: usize = 0;
    for (i, item) in self.get_chars().enumerate() {
      if i == chars {
        break;
      } else {
        diff += item.len_utf8();
      }
    }

    self.advance_bytes(diff)
  }

  /// Unsafe, panic expected if bytes wrong
  pub fn advance_bytes(&mut self, bytes: usize) {
    let target = self.pointer + bytes;
    self.pointer = target;
  }

  /// Get the next char without moving the pointer
  pub fn peek(&self) -> Option<char> {
    self.get_chars().nth(1)
  }
}
