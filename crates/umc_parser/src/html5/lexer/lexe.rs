use std::iter::from_fn;

use crate::html5::lexer::{Html5Lexer, token::Html5Token};

impl<'a> Html5Lexer<'a> {
  pub fn tokens(&'a self) -> impl Iterator<Item = Html5Token> + 'a {
    from_fn(|| self.next_token())
  }

  pub fn next_token(&'a self) -> Option<Html5Token> {
    Some(Html5Token::default())
  }
}

mod test {
  use oxc_allocator::Allocator;

  use super::Html5Lexer;

  const HTML_STRING: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  
</body>
</html>"#;

  #[test]
  fn get_tokens() {
    println!(Html5Lexer::new(Allocator::default()).tokens().collect())
  }
}
