use oxc_diagnostics::{LabeledSpan, OxcDiagnostic};

use crate::html5::lexer::{
  Html5Lexer, LexerStateKind,
  kind::Html5Kind,
  token::{Html5Token, Html5TokenValue},
};
use std::{iter::from_fn, str::Chars};

impl<'a> Html5Lexer<'a> {
  pub fn tokens(&mut self) -> impl Iterator<Item = Html5Token> {
    from_fn(move || self.next_token())
  }

  /// Get the next token, and move the pointer
  fn next_token(&mut self) -> Option<Html5Token> {
    // the file end, but still calling this function
    if self.is_eof() {
      return match self.state.kind {
        LexerStateKind::Finished => None,
        _ => Some(self.finish()),
      };
    }

    // match the state and do different lexing
    match self.state.kind {
      LexerStateKind::Content => Some(self.handle_content()),
      LexerStateKind::EmbeddedContent => Some(self.handle_embedded_content()),
      LexerStateKind::AfterTagName => Some(self.handle_after_tag_name()),
      LexerStateKind::InTag => Some(self.handle_in_tag()),
      LexerStateKind::Finished => None,
    }
  }

  #[inline]
  fn is_eof(&self) -> bool {
    self.source.pointer >= self.source.source_text.len()
  }

  #[inline]
  fn finish(&mut self) -> Html5Token {
    self.state.kind = LexerStateKind::Finished; // mark as finished

    Html5Token {
      kind: Html5Kind::Eof,
      start: self.source.pointer,
      end: self.source.pointer,
      value: Html5TokenValue::None,
    }
  }
}

// handler for Html5LexerState::Content
impl<'a> Html5Lexer<'a> {
  fn handle_content(&mut self) -> Html5Token {
    let mut iter: Chars<'_> = self.source.get_chars();
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
            self.state.kind = LexerStateKind::InTag; // update state
            self.state.allow_to_set_tag_name();
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
            self.state.kind = LexerStateKind::InTag; // update state
            result
          }

          // for ! character, as comment or doctype
          Some('!') => {
            diff += '!'.len_utf8();

            const COMMENT_START: [char; 2] = ['-', '-'];
            const DOCTYPE_START: [char; 7] = ['D', 'O', 'C', 'T', 'Y', 'P', 'E'];
            let mut match_doctype = true;
            let mut match_commement = true;
            let mut i = 0;

            while let Some(item) = iter.next() {
              diff += item.len_utf8();

              if match_doctype && DOCTYPE_START.get(i) == Some(&item) {
                if i == DOCTYPE_START.len() - 1 {
                  // it's a doctype
                  let result = Html5Token {
                    kind: Html5Kind::Doctype,
                    start: self.source.pointer,
                    end: self.source.pointer + diff,
                    value: Html5TokenValue::None,
                  };

                  self.source.advance_bytes(diff);
                  self.state.kind = LexerStateKind::AfterTagName; // update state

                  return result;
                }
              } else {
                match_doctype = false;
              }

              if match_commement && COMMENT_START.get(i) == Some(&item) {
                if i == COMMENT_START.len() {
                  // it's a comment
                  return self.handle_comment(&mut iter, &mut diff);
                }
              } else {
                match_commement = false;
              }

              if !match_doctype && !match_commement {
                // it is neither doctype nor comment, treat as bogus comment (ends with > instead of -->)
                return self.handle_bogus_comment(&mut iter, &mut diff);
              }

              i += 1
            }

            self.tailless_comment(diff)
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

  fn handle_bogus_comment(&mut self, iter: &mut Chars, diff: &mut usize) -> Html5Token {
    let mut ended = false;
    for item in iter {
      *diff += item.len_utf8();

      if item == '>' {
        ended = true;
        break;
      }
    }

    if !ended {
      // eof without finishing doctype or comment
      return self.tailless_comment(*diff);
    }

    let result = Html5Token {
      kind: Html5Kind::Comment,
      start: self.source.pointer,
      end: self.source.pointer + *diff,
      value: Html5TokenValue::String({
        let raw_text = &self.source.source_text[self.source.pointer..self.source.pointer + *diff];
        // the struct: <! something >
        raw_text[2..raw_text.len() - 2].to_owned()
      }),
    };

    self.source.advance_bytes(*diff); // It still on Content state like this: sometest|<! something >| moretext
    result
  }

  fn handle_comment(&mut self, iter: &mut Chars, diff: &mut usize) -> Html5Token {
    let mut dash_count: u8 = 0;
    let mut ended = false;

    for item in iter {
      *diff += item.len_utf8();

      match item {
        '-' => {
          dash_count += 1;
        }
        '>' => {
          if dash_count >= 2 {
            // comment ended
            ended = true;
            break;
          } else {
            dash_count = 0; // reset dash count
          }
        }
        _ => {
          dash_count = 0; // reset dash count
        }
      }
    }

    if !ended {
      // eof without finishing doctype or comment
      return self.tailless_comment(*diff);
    }

    let result = Html5Token {
      kind: Html5Kind::Comment,
      start: self.source.pointer,
      end: self.source.pointer + *diff,
      value: Html5TokenValue::String({
        let raw_text = &self.source.source_text[self.source.pointer..self.source.pointer + *diff];
        // the struct: <!-- something -->
        raw_text[4..raw_text.len() - 3].to_owned()
      }),
    };

    self.source.advance_bytes(*diff); // It still on Content state like this: sometest|<!-- something -->| moretext
    result
  }

  fn tailless_comment(&mut self, diff: usize) -> Html5Token {
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
        let raw_text = &self.source.source_text[self.source.pointer..self.source.pointer + diff];
        if let Some(comment) = raw_text.strip_prefix("<!--") {
          comment.to_owned()
        } else {
          raw_text[2..].to_owned()
        }
      }),
    };

    self.source.advance_bytes(diff);
    self.state.kind = LexerStateKind::AfterTagName; // update state
    result
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

// handler for Html5LexerState::EmbeddedContent
impl<'a> Html5Lexer<'a> {
  fn handle_embedded_content(&mut self) -> Html5Token {
    let mut diff: usize = 0;
    let closing_tag = format!("</{}", self.state.take_tag_name().unwrap()); // safe unwrap because only script/style can enter this state
    let mut ended = false;

    for item in self.source.get_chars() {
      diff += item.len_utf8();

      if self.source.source_text[self.source.pointer + diff..].starts_with(&closing_tag) {
        ended = true;
        break;
      }
    }

    if !ended {
      // throw an error, expect closing tag, but found eof
      let error_message = format!("Expected {}, but found {}", closing_tag, Html5Kind::Eof,);
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
        self.source.source_text[self.source.pointer..self.source.pointer + diff].to_owned(),
      ),
      kind: Html5Kind::TextContent,
    };
    self.source.advance_bytes(diff);
    self.state.kind = LexerStateKind::Content; // update state
    result
  }
}

// handler for Html5LexerState::AfterTagName
impl<'a> Html5Lexer<'a> {
  fn handle_after_tag_name(&mut self) -> Html5Token {
    let mut iter: Chars<'_> = self.source.get_chars();

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

        // update state
        const EMBEDDED_LANGUAGE_TAG: [&str; 2] = ["script", "style"];
        if let Some(tag_name) = self.state.get_tag_name()
          && EMBEDDED_LANGUAGE_TAG.contains(&tag_name)
        {
          self.state.kind = LexerStateKind::EmbeddedContent;
        } else {
          self.state.kind = LexerStateKind::Content;
        }

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
            self.state.take_tag_name(); // clear tag name
            self.state.kind = LexerStateKind::Content; // update state
            result
          }
          None | Some(_) => self.handle_tag(&mut iter, &mut diff, Html5Kind::Attribute),
        }
      }

      // for attribute with `"`
      '"' => self.handle_quote_attribute(&mut iter, '"'),

      // for attribute with `'`
      '\'' => self.handle_quote_attribute(&mut iter, '\''),

      // for attribute without `"`
      c => {
        let mut diff = c.len_utf8();
        self.handle_tag(&mut iter, &mut diff, Html5Kind::Attribute)
      }
    }
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

// handler for Html5LexerState::InTag
impl<'a> Html5Lexer<'a> {
  fn handle_in_tag(&mut self) -> Html5Token {
    // call the handle_tag
    let mut iter = self.source.get_chars();
    let mut diff: usize = 0;

    let result = self.handle_tag(&mut iter, &mut diff, Html5Kind::ElementName);
    self.state.kind = LexerStateKind::AfterTagName; // update state
    self
      .state
      .set_tag_name(self.source.source_text[result.range()].to_owned());
    result
  }
}

// some universal functions
impl<'a> Html5Lexer<'a> {
  fn handle_tag(&mut self, iter: &mut Chars, diff: &mut usize, kind: Html5Kind) -> Html5Token {
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
      kind,
    };

    self.source.advance_bytes(*diff);
    result
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::html5::lexer::{LexerState, source::Source};
  use oxc_allocator::Allocator;

  #[test]
  fn after_tag_name_should_work() {
    const SOURCE_TEXT: &str = r#" class="w-full h-full" p-1
 复杂字段测试 /test "alpha"
       />"#;
    let mut lexer = Html5Lexer {
      _allocator: &Allocator::default(),
      source: Source::new(SOURCE_TEXT),
      state: LexerState::new(LexerStateKind::AfterTagName),
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
      _allocator: &Allocator::default(),
      source: Source::new(SOURCE_TEXT),
      state: LexerState::new(LexerStateKind::AfterTagName),
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
