use crate::html5::{Html5ParserOptions, parse};
use oxc_allocator::Allocator;

mod filename;
mod html5;

pub enum ParserOptions {
  Html5(Html5ParserOptions),
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

  pub fn parse(&self) {
    match &self.options {
      ParserOptions::Html5(options) => parse(self, options),
    };
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_create_parser() {
    let allocator = Allocator::default();
    let _parser = Parser::new(
      &allocator,
      r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  
</body>
</html>
  "#,
      ParserOptions::default_from_filename("index.html"),
    );
  }
}
