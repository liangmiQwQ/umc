use crate::html5::lexer::{
  Html5Lexer, Html5LexerState,
  kind::Html5Kind,
  token::{Html5Token, Html5TokenValue},
};
use std::{iter::from_fn, str::Chars};

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

// handler for Html5LexerState::AfterTagName
impl<'a> Html5Lexer<'a> {
  fn handle_after_tag_name(&mut self) -> Html5Token {
    let mut iter: std::str::Chars<'_> = self.source.get_chars();

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
        self.state = Html5LexerState::Content; // update state
        result
      }

      // for self close end and attribute starts with `/`
      '/' => {
        let mut diff = '/'.len_utf8();

        if let Some(next) = iter.next() // don't unwrap because the file may end with `/`
          && next == '>'
        {
          // self close
          diff += '>'.len_utf8();

          let result = Html5Token {
            kind: Html5Kind::SelfCloseTagEnd,
            start: self.source.pointer,
            end: self.source.pointer + diff,
            value: Html5TokenValue::None,
          };

          self.source.advance_bytes(diff);
          self.state = Html5LexerState::Content; // update state
          result
        } else {
          // the attribute starts with '/'
          self.handle_no_quote_attribute(&mut iter, &mut diff)
        }
      }

      // for attribute with `"`
      '"' => self.handle_quote_attribute(&mut iter, '"'),

      // for attribute with `'`
      '\'' => self.handle_quote_attribute(&mut iter, '\''),

      // for attribute without `"`
      c => {
        let mut diff = c.len_utf8();
        self.handle_no_quote_attribute(&mut iter, &mut diff)
      }
    }
  }

  fn handle_no_quote_attribute(&mut self, iter: &mut Chars, diff: &mut usize) -> Html5Token {
    for item in iter {
      if item.is_whitespace() || item == '>' || item == '=' {
        // end of a attribute
        break;
      } else {
        *diff += item.len_utf8();
      }
    }

    let result = Html5Token {
      start: self.source.pointer,
      end: self.source.pointer + *diff,
      value: Html5TokenValue::String(
        self.source.source_text[self.source.pointer..self.source.pointer + *diff].to_owned(),
      ),
      kind: Html5Kind::Attribute,
    };

    self.source.advance_bytes(*diff);
    result
  }

  fn handle_quote_attribute(&mut self, iter: &mut Chars, quote: char) -> Html5Token {
    // since html don't support \ escape, we don't need to manage its state
    let mut diff = quote.len_utf8();

    for item in iter {
      diff += item.len_utf8();

      match item {
        c if c == quote => break, // the string is ended
        _ => (),
      }
    }

    let result = Html5Token {
      start: self.source.pointer,
      end: self.source.pointer + diff,
      value: Html5TokenValue::String(
        self.source.source_text[self.source.pointer..self.source.pointer + diff]
          .trim_start_matches(quote)
          .trim_end_matches(quote)
          .to_owned(),
      ),
      kind: Html5Kind::Attribute,
    };

    self.source.advance_bytes(diff);
    result
  }
}
