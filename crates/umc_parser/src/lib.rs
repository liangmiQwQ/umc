use oxc_allocator::Allocator;

pub struct Parser<'a> {
  allocator: &'a Allocator,
  source_text: &'a str,
}

impl<'a> Parser<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Self {
      allocator,
      source_text,
    }
  }
}

#[cfg(test)]
mod test {

  #[test]
  fn test_create_parser() {
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, "some code there");
  }
}
