use oxc_allocator::Allocator;
mod filename;

pub enum ParserOptions {
  Html5 {},
}

pub struct Parser<'a> {
  allocator: &'a Allocator,
  source_text: &'a str,
  options: ParserOptions,
}

impl<'a> Parser<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str, options: ParserOptions) -> Self {
    Self {
      allocator,
      source_text,
      options,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_create_parser() {
    let allocator = Allocator::default();
    let parser = Parser::new(
      &allocator,
      "some code there",
      ParserOptions::default_from_filename("index.html"),
    );
  }
}
