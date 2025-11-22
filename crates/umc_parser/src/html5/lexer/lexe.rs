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

  /// Get the next token, and move the pointer
  fn next_token(&mut self) -> Option<Html5Token> {
    // the file end, but still calling this function
    if self.is_eof() {
      return match self.state {
        Html5LexerState::Finished => None,
        _ => Some(self.finish()),
      };
    }

    // match the state and do different lexing
    match self.state {
      Html5LexerState::AfterTagName => Some(self.handle_after_tag_name()),
      _ => None,
    }
  }

  fn handle_after_tag_name(&mut self) -> Html5Token {
    let mut iter = self.source.get_chars();

    // safe unwarp, won't direct to this branch if pointer == file.len()
    match iter.next().unwrap() {
      // for whitespace
      c if c.is_whitespace() => {
        let mut diff: usize = c.len_utf8();
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

      // for =
      '=' => {
        let diff = '='.len_utf8();

        let result = Html5Token {
          kind: Html5Kind::Eq,
          start: self.source.pointer,
          end: self.source.pointer + diff,
          value: Html5TokenValue::None,
        };

        self.source.advance_bytes(diff);
        result
      }

      // for tag end (>)
      '>' => {
        let diff = '>'.len_utf8();

        let result = Html5Token {
          kind: Html5Kind::TagEnd,
          start: self.source.pointer,
          end: self.source.pointer + diff,
          value: Html5TokenValue::None,
        };

        self.source.advance_bytes(diff);
        result
      }

      // for self close end and attribute starts with `/`
      '/' => {
        let diff = '/'.len_utf8();

        let result = Html5Token {
          kind: Html5Kind::TagEnd,
          start: self.source.pointer,
          end: self.source.pointer + diff,
          value: Html5TokenValue::None,
        };

        self.source.advance_bytes(diff);
        result
      }

      // for attribute with `"`
      '"' => todo!(),

      // for attribute with `'`
      '\'' => todo!(),

      // for attribute without `"`
      _ => todo!(),
    }
  }

  #[inline]
  fn is_eof(&self) -> bool {
    self.source.pointer >= self.source.source_text.len()
  }

  #[inline]
  fn finish(&mut self) -> Html5Token {
    self.state = Html5LexerState::Finished; // mark as finished

    Html5Token {
      kind: Html5Kind::Eof,
      start: self.source.pointer,
      end: self.source.pointer,
      value: Html5TokenValue::None,
    }
  }
}
