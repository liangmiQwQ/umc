use oxc_diagnostics::{LabeledSpan, OxcDiagnostic};

use crate::html5::lexer::{
  Html5Lexer, Html5LexerState,
  kind::Html5Kind,
  token::{Html5Token, Html5TokenValue},
};
use std::{iter::from_fn, os::macos::raw, str::Chars};

impl<'a> Html5Lexer<'a> {
  pub fn tokens(&mut self) -> impl Iterator<Item = Html5Token> {
    from_fn(move || self.next_token())
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
      Html5LexerState::Content => Some(self.handle_content()),
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

        let result = {
          let result = iter.next();
          if let Some(next) = result {
            diff += next.len_utf8();
          }
          result
        };

        match result {
          Some('>') => {
            // self close
            let result = Html5Token {
              kind: Html5Kind::SelfCloseTagEnd,
              start: self.source.pointer,
              end: self.source.pointer + diff,
              value: Html5TokenValue::None,
            };

            self.source.advance_bytes(diff);
            self.state = Html5LexerState::Content; // update state
            result
          }
          None | Some(_) => self.handle_no_quote_attribute(&mut iter, &mut diff),
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
      if item.is_whitespace() || item == '>' || item == '=' || item == '/' {
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
    let mut ended = false;

    for item in iter {
      diff += item.len_utf8();

      match item {
        c if c == quote => {
          ended = true;
          break;
        } // the string is ended
        _ => (),
      }
    }

    if !ended {
      // throw an error, expect quote, but found eof
      let error_message = format!("Expected {}, but found {}", quote, Html5Kind::Eof,);
      let label = LabeledSpan::at(
        self.source.pointer + diff - 1..self.source.pointer + diff,
        &error_message,
      );

      self
        .errors
        .push(OxcDiagnostic::error(error_message).with_label(label));
    }

    let result = Html5Token {
      start: self.source.pointer,
      end: self.source.pointer + diff,
      value: Html5TokenValue::String(
        // do not need to remove quote because we need it
        self.source.source_text[self.source.pointer..self.source.pointer + diff].to_owned(),
      ),
      kind: Html5Kind::Attribute,
    };

    self.source.advance_bytes(diff);
    result
  }
}

// handler for Html5LexerState::Content
impl<'a> Html5Lexer<'a> {
  fn handle_content(&mut self) -> Html5Token {
    let mut iter: std::str::Chars<'_> = self.source.get_chars();
    // safe unwarp, won't direct to this branch if pointer == file.len()
    match iter.next().unwrap() {
      // for <
      '<' => {
        // maybe comment, doctype, tag or < starting content
        let mut diff: usize = '<'.len_utf8();

        match iter.next() {
          // for alphabetic character, as tag start
          Some(item) if item.is_alphabetic() => {
            // do not need to add diff, because we only need the < part
            let result = Html5Token {
              kind: Html5Kind::TagStart,
              start: self.source.pointer,
              end: self.source.pointer + diff,
              value: Html5TokenValue::None,
            };

            self.source.advance_bytes(diff);
            self.state = Html5LexerState::InTag; // update state
            result
          }

          // for / character, as closing tag
          Some('/') => {
            diff += '/'.len_utf8();

            let result = Html5Token {
              kind: Html5Kind::CloseTagStart,
              start: self.source.pointer,
              end: self.source.pointer + diff,
              value: Html5TokenValue::None,
            };

            self.source.advance_bytes(diff);
            self.state = Html5LexerState::InTag; // update state
            result
          }

          // for ! character, as comment or doctype
          Some('!') => {
            const COMMENT_START: [char; 2] = ['-', '-'];
            const DOCTYPE_START: [char; 7] = ['D', 'O', 'C', 'T', 'Y', 'P', 'E'];
            let mut match_doctype = true;
            let mut match_commement = true;

            for (i, item) in iter.enumerate() {
              diff += item.len_utf8();

              if match_doctype && DOCTYPE_START.get(i) == Some(&item) {
                if i == DOCTYPE_START.len() {
                  // it's a doctype
                  let result = Html5Token {
                    kind: Html5Kind::Doctype,
                    start: self.source.pointer,
                    end: self.source.pointer + diff,
                    value: Html5TokenValue::None,
                  };

                  self.source.advance_bytes(diff);
                  self.state = Html5LexerState::AfterTagName; // update state

                  return result;
                }
              } else {
                match_doctype = false;
              }

              if match_commement && COMMENT_START.get(i) == Some(&item) {
                if i == COMMENT_START.len() {
                  // it's a comment
                  todo!()
                }
              } else {
                match_commement = false;
              }

              if !match_doctype && !match_commement {
                // it is neither doctype nor comment, treat as fake comment (ends with > instead of -->)
              }
            }

            // eof without finishing doctype or comment
            // throw an error
            let error_message = format!(
              "Expected {}, but found {}",
              Html5Kind::TagEnd,
              Html5Kind::Eof,
            );
            let label = LabeledSpan::at(
              self.source.pointer + diff - 1..self.source.pointer + diff,
              &error_message,
            );
            self
              .errors
              .push(OxcDiagnostic::error(error_message).with_label(label));

            // return as comment
            let result = Html5Token {
              kind: Html5Kind::Comment,
              start: self.source.pointer,
              end: self.source.pointer + diff,
              value: Html5TokenValue::String({
                let raw_text =
                  &self.source.source_text[self.source.pointer..self.source.pointer + diff];
                if let Some(comment) = raw_text.strip_prefix("<!--") {
                  comment.to_owned()
                } else {
                  raw_text[2..].to_owned()
                }
              }),
            };

            self.source.advance_bytes(diff);
            self.state = Html5LexerState::AfterTagName; // update state
            result
          }

          // for none and other character, as content starting with <
          None | Some(_) => {
            // record until next tag start
            self.handle_content_text(&mut iter, &mut diff)
          }
        }
      }

      // for content
      c => {
        // record until next tag start
        let mut diff: usize = c.len_utf8();
        self.handle_content_text(&mut iter, &mut diff)
      }
    }
  }

  fn handle_content_text(&mut self, iter: &mut Chars, diff: &mut usize) -> Html5Token {
    let mut check_next = false;

    for item in iter {
      if check_next {
        if item.is_alphabetic() || item == '/' || item == '!' {
          break;
        } else {
          *diff += item.len_utf8() + '<'.len_utf8();
          check_next = false;
          continue;
        }
      }

      if item == '<' {
        check_next = true;
      } else {
        *diff += item.len_utf8();
      }
    }

    let result = Html5Token {
      kind: Html5Kind::TextContent,
      start: self.source.pointer,
      end: self.source.pointer + *diff,
      value: Html5TokenValue::String(
        self.source.source_text[self.source.pointer..self.source.pointer + *diff].to_owned(),
      ),
    };

    self.source.advance_bytes(*diff);
    result
  }
}

#[cfg(test)]
mod test {
  use super::{Html5Lexer, Html5LexerState, Html5Token};
  use crate::html5::lexer::source::Source;
  use oxc_allocator::Allocator;

  #[test]
  fn after_tag_name_should_work() {
    const SOURCE_TEXT: &str = r#" class="w-full h-full" p-1
 复杂字段测试 /test "alpha"
       />"#;
    let mut lexer = Html5Lexer {
      allocator: &Allocator::default(),
      source: Source::new(SOURCE_TEXT),
      state: Html5LexerState::AfterTagName,
      errors: Vec::new(),
    };

    let tokens: Vec<Html5Token> = lexer.tokens().collect();
    insta::assert_snapshot!(format!(
      "Source: {:#?}; \nTokens:{:#?}",
      SOURCE_TEXT, tokens
    ));
  }

  #[test]
  fn after_tag_name_should_return_error() {
    const SOURCE_TEXT: &str = r#" class="w-full"#;
    let mut lexer = Html5Lexer {
      allocator: &Allocator::default(),
      source: Source::new(SOURCE_TEXT),
      state: Html5LexerState::AfterTagName,
      errors: Vec::new(),
    };

    lexer.tokens().for_each(drop);

    insta::assert_snapshot!(format!(
      "Source: {:#?}; \nErrors:{:#?}",
      SOURCE_TEXT,
      lexer.errors.get(0).unwrap()
    ));
  }
}
