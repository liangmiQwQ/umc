use crate::html5::lexer::{Html5Lexer, token::Html5Token};
use std::iter::from_fn;

impl<'a> Html5Lexer<'a> {
  pub fn tokens(&'a self) -> impl Iterator<Item = Html5Token> + 'a {
    from_fn(|| self.next_token())
  }

  fn next_token(&'a self) -> Option<Html5Token> {
    let a = match self.source.current()? {
      ' ' => "",
      _ => "b",
    };
  }
}

#[cfg(test)]
mod test {
  use super::Html5Lexer;
  use crate::html5::lexer::token::Html5Token;
  use oxc_allocator::Allocator;

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
    let result: Vec<Html5Token> = Html5Lexer::new(&Allocator::default(), HTML_STRING)
      .tokens()
      .collect();

    println!("{:#?}", result);

    assert_eq!(1, 1)
  }
}
