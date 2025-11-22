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
  pub fn current(&self) -> Option<char> {
    self.source_text[self.pointer..].chars().next()
  }

  pub fn next(&mut self) -> Option<char> {
    let current = self.current()?;
    self.pointer += current.len_utf8();
    Some(current)
  }

  // Get the next char without moving the pointer
  pub fn peek(&self) -> Option<char> {
    self.source_text[self.pointer..].chars().nth(1)
  }
}
