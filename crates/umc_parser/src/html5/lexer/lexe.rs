use crate::html5::lexer::{Html5Lexer, token::Html5Token};

impl<'a> Html5Lexer<'a> {
  pub fn tokens(&'a self) -> impl Iterator<Item = Html5Token> + 'a {
    std::iter::from_fn(|| self.next_token())
  }

  pub fn next_token(&'a self) -> Option<Html5Token> {
    Some(Html5Token::default())
  }
}
