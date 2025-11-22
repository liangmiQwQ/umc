use crate::html5::lexer::{
  Html5Lexer, Html5LexerState,
  kind::Html5Kind,
  token::{Html5Token, Html5TokenValue},
};
use std::iter::from_fn;

impl<'a> Html5Lexer<'a> {
  pub fn tokens(&'a mut self) -> impl Iterator<Item = Html5Token> + 'a {
    from_fn(|| self.next_token())
  }

  fn next_token(&mut self) -> Option<Html5Token> {
    if self.source.pointer >= self.source.source_text.len() {
      match self.state {
        Html5LexerState::Finished => None,
        _ => Some(Html5Token {
          kind: Html5Kind::Eof,
          start: self.source.pointer,
          end: self.source.pointer,
          value: Html5TokenValue::None,
        }),
      }
    } else {
      match self.source.current()? {
        c if c.is_whitespace() => Some(self.get_whitespace_token()),
        _ => None,
      }
    }
  }

  fn get_whitespace_token(&mut self) -> Html5Token {
    let iter = self.source.get_chars();

    let mut diff: usize = 0;
    for item in iter {
      if item.is_whitespace() {
        diff += item.len_utf8();
      } else {
        break;
      }
    }

    let result = Html5Token {
      start: self.source.pointer,
      end: self.source.pointer + diff,
      value: Html5TokenValue::None,
      kind: Html5Kind::Whitespace,
    };

    self.source.advance_bytes(diff);
    result
  }
}

#[cfg(test)]
mod test {
  use super::{Html5Kind, Html5Lexer, Html5Token, Html5TokenValue};
  use oxc_allocator::Allocator;

  const HTML_STRING: &str = r#"      <!DOCTYPE html>
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

    assert_eq!(
      result,
      vec![Html5Token {
        kind: Html5Kind::Whitespace,
        start: 0,
        end: 6,
        value: Html5TokenValue::None
      }]
    )
  }
}
