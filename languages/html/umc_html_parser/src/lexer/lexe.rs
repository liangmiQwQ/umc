use oxc_diagnostics::OxcDiagnostic;
use std::{iter::from_fn, str::Chars};
use umc_parser::{char::len_utf8_u32, token::Token};
use umc_span::Span;

use crate::lexer::{HtmlLexer, kind::HtmlKind, state::LexerStateKind};

impl<'a> HtmlLexer<'a> {
  pub fn tokens(&mut self) -> impl Iterator<Item = Token<HtmlKind>> {
    from_fn(move || self.next_token())
  }

  /// Get the next token, and move the pointer
  fn next_token(&mut self) -> Option<Token<HtmlKind>> {
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
    self.source.pointer as usize >= self.source.source_text.len()
  }

  #[inline]
  fn finish(&mut self) -> Token<HtmlKind> {
    self.state.kind = LexerStateKind::Finished; // mark as finished

    Token::<HtmlKind> {
      kind: HtmlKind::Eof,
      start: self.source.pointer,
      end: self.source.pointer,
    }
  }
}

// handler for HtmlLexerState::Content
impl<'a> HtmlLexer<'a> {
  fn handle_content(&mut self) -> Token<HtmlKind> {
    let mut iter: Chars<'_> = self.source.get_chars();
    // safe unwarp, won't direct to this branch if pointer == file.len()
    match iter.next().unwrap() {
      // for <
      '<' => {
        // maybe comment, doctype, tag or < starting content
        let mut diff: u32 = len_utf8_u32('<');

        match iter.next() {
          // for alphabetic character, as tag start
          Some(item) if item.is_alphabetic() => {
            // do not need to add diff, because we only need the < part
            let result = Token::<HtmlKind> {
              kind: HtmlKind::TagStart,
              start: self.source.pointer,
              end: self.source.pointer + diff,
            };

            self.source.advance_bytes(diff);
            self.state.kind = LexerStateKind::InTag; // update state
            self.state.allow_to_set_tag_name();
            result
          }

          // for / character, as closing tag
          Some('/') => {
            diff += len_utf8_u32('/');

            let result = Token::<HtmlKind> {
              kind: HtmlKind::CloseTagStart,
              start: self.source.pointer,
              end: self.source.pointer + diff,
            };

            self.source.advance_bytes(diff);
            self.state.kind = LexerStateKind::InTag; // update state
            result
          }

          // for ! character, as comment or doctype
          Some('!') => {
            diff += len_utf8_u32('!');

            const COMMENT_START: [char; 2] = ['-', '-'];
            const DOCTYPE_START: [char; 7] = ['D', 'O', 'C', 'T', 'Y', 'P', 'E'];
            let mut match_doctype = true;
            let mut match_commement = true;
            let mut i = 0;

            while let Some(item) = iter.next() {
              diff += len_utf8_u32(item);

              if match_doctype && DOCTYPE_START.get(i) == Some(&item) {
                if i == DOCTYPE_START.len() - 1 {
                  // it's a doctype
                  let result = Token::<HtmlKind> {
                    kind: HtmlKind::Doctype,
                    start: self.source.pointer,
                    end: self.source.pointer + diff,
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
        let mut diff: u32 = len_utf8_u32(c);
        self.handle_content_text(&mut iter, &mut diff)
      }
    }
  }

  fn handle_bogus_comment(&mut self, iter: &mut Chars, diff: &mut u32) -> Token<HtmlKind> {
    let mut ended = false;
    for item in iter {
      *diff += len_utf8_u32(item);

      if item == '>' {
        ended = true;
        break;
      }
    }

    if !ended {
      // eof without finishing doctype or comment
      return self.tailless_comment(*diff);
    }

    let result = Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start: self.source.pointer,
      end: self.source.pointer + *diff,
    };

    self.source.advance_bytes(*diff); // It still on Content state like this: sometest|<! something >| moretext
    result
  }

  fn handle_comment(&mut self, iter: &mut Chars, diff: &mut u32) -> Token<HtmlKind> {
    let mut dash_count: u8 = 0;
    let mut ended = false;

    for item in iter {
      *diff += len_utf8_u32(item);

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

    let result = Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start: self.source.pointer,
      end: self.source.pointer + *diff,
    };

    self.source.advance_bytes(*diff); // It still on Content state like this: sometest|<!-- something -->| moretext
    result
  }

  fn tailless_comment(&mut self, diff: u32) -> Token<HtmlKind> {
    // eof without finishing doctype or comment
    // throw an error
    self.errors.push(
      OxcDiagnostic::error(format!(
        "Expected {}, but found {}",
        HtmlKind::TagEnd,
        HtmlKind::Eof
      ))
      .with_label(Span::new(
        self.source.pointer + diff,
        self.source.pointer + diff,
      )),
    );

    // return as comment
    let result = Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start: self.source.pointer,
      end: self.source.pointer + diff,
    };

    self.source.advance_bytes(diff);
    self.state.kind = LexerStateKind::AfterTagName; // update state
    result
  }

  fn handle_content_text(&mut self, iter: &mut Chars, diff: &mut u32) -> Token<HtmlKind> {
    let mut check_next = false;

    for item in iter {
      if check_next {
        if item.is_alphabetic() || item == '/' || item == '!' {
          break;
        } else {
          *diff += len_utf8_u32(item) + len_utf8_u32('<');
          check_next = false;
          continue;
        }
      }

      if item == '<' {
        check_next = true;
      } else {
        *diff += len_utf8_u32(item);
      }
    }

    let result = Token::<HtmlKind> {
      kind: HtmlKind::TextContent,
      start: self.source.pointer,
      end: self.source.pointer + *diff,
    };

    self.source.advance_bytes(*diff);
    result
  }
}

// handler for HtmlLexerState::EmbeddedContent
impl<'a> HtmlLexer<'a> {
  fn handle_embedded_content(&mut self) -> Token<HtmlKind> {
    let mut diff: u32 = 0;
    let closing_tag = format!("</{}", self.state.take_tag_name().unwrap()); // safe unwrap because only script/style can enter this state
    let mut ended = false;

    for item in self.source.get_chars() {
      diff += len_utf8_u32(item);

      if self.source.source_text[(self.source.pointer + diff) as usize..].starts_with(&closing_tag)
      {
        ended = true;
        break;
      }
    }

    if !ended {
      // throw an error, expect closing tag, but found eof
      self.errors.push(
        OxcDiagnostic::error(format!(
          "Expected {}, but found {}",
          closing_tag,
          HtmlKind::Eof
        ))
        .with_label(Span::new(
          self.source.pointer + diff,
          self.source.pointer + diff,
        )),
      );
    }

    let result = Token::<HtmlKind> {
      start: self.source.pointer,
      end: self.source.pointer + diff,
      kind: HtmlKind::TextContent,
    };
    self.source.advance_bytes(diff);
    self.state.kind = LexerStateKind::Content; // update state
    result
  }
}

// handler for HtmlLexerState::AfterTagName
impl<'a> HtmlLexer<'a> {
  fn handle_after_tag_name(&mut self) -> Token<HtmlKind> {
    let mut iter: Chars<'_> = self.source.get_chars();

    // safe unwarp, won't direct to this branch if pointer == file.len()
    match iter.next().unwrap() {
      // for whitespace
      c if c.is_whitespace() => {
        let mut diff: u32 = len_utf8_u32(c);

        for item in iter {
          if item.is_whitespace() {
            diff += len_utf8_u32(item);
          } else {
            break;
          }
        }

        let result = Token::<HtmlKind> {
          start: self.source.pointer,
          end: self.source.pointer + diff,
          kind: HtmlKind::Whitespace,
        };

        self.source.advance_bytes(diff);
        result
      }

      // for =
      '=' => {
        let diff = len_utf8_u32('=');

        let result = Token::<HtmlKind> {
          kind: HtmlKind::Eq,
          start: self.source.pointer,
          end: self.source.pointer + diff,
        };

        self.source.advance_bytes(diff);
        result
      }

      // for tag end (>)
      '>' => {
        let diff = len_utf8_u32('>');

        let result = Token::<HtmlKind> {
          kind: HtmlKind::TagEnd,
          start: self.source.pointer,
          end: self.source.pointer + diff,
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
        let mut diff = len_utf8_u32('/');

        let result = {
          let result = iter.next();
          if let Some(next) = result {
            diff += len_utf8_u32(next);
          }
          result
        };

        match result {
          Some('>') => {
            // self close
            let result = Token::<HtmlKind> {
              kind: HtmlKind::SelfCloseTagEnd,
              start: self.source.pointer,
              end: self.source.pointer + diff,
            };

            self.source.advance_bytes(diff);
            self.state.take_tag_name(); // clear tag name
            self.state.kind = LexerStateKind::Content; // update state
            result
          }
          None | Some(_) => self.handle_tag(&mut iter, &mut diff, HtmlKind::Attribute),
        }
      }

      // for attribute with `"`
      '"' => self.handle_quote_attribute(&mut iter, '"'),

      // for attribute with `'`
      '\'' => self.handle_quote_attribute(&mut iter, '\''),

      // for attribute without `"`
      c => {
        let mut diff = len_utf8_u32(c);
        self.handle_tag(&mut iter, &mut diff, HtmlKind::Attribute)
      }
    }
  }

  fn handle_quote_attribute(&mut self, iter: &mut Chars, quote: char) -> Token<HtmlKind> {
    // since html don't support \ escape, we don't need to manage its state
    let mut diff = len_utf8_u32(quote);
    let mut ended = false;

    for item in iter {
      diff += len_utf8_u32(item);

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
      self.errors.push(
        OxcDiagnostic::error(format!("Expected {}, but found {}", quote, HtmlKind::Eof))
          .with_label(Span::new(
            self.source.pointer + diff,
            self.source.pointer + diff,
          )),
      );
    }

    let result = Token::<HtmlKind> {
      start: self.source.pointer,
      end: self.source.pointer + diff,
      kind: HtmlKind::Attribute,
    };

    self.source.advance_bytes(diff);
    result
  }
}

// handler for HtmlLexerState::InTag
impl<'a> HtmlLexer<'a> {
  fn handle_in_tag(&mut self) -> Token<HtmlKind> {
    // call the handle_tag
    let mut iter = self.source.get_chars();
    let mut diff: u32 = 0;

    let result = self.handle_tag(&mut iter, &mut diff, HtmlKind::ElementName);
    self.state.kind = LexerStateKind::AfterTagName; // update state
    self
      .state
      .set_tag_name(self.source.source_text[result.start as usize..result.end as usize].to_owned());
    result
  }
}

// some universal functions
impl<'a> HtmlLexer<'a> {
  fn handle_tag(&mut self, iter: &mut Chars, diff: &mut u32, kind: HtmlKind) -> Token<HtmlKind> {
    for item in iter {
      if item.is_whitespace() || item == '>' || item == '=' || item == '/' {
        // end of a attribute
        break;
      } else {
        *diff += len_utf8_u32(item);
      }
    }

    let result = Token::<HtmlKind> {
      start: self.source.pointer,
      end: self.source.pointer + *diff,
      kind,
    };

    self.source.advance_bytes(*diff);
    result
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::lexer::LexerState;
  use oxc_allocator::Allocator;
  use umc_parser::source::Source;

  #[test]
  fn after_tag_name_should_work() {
    const SOURCE_TEXT: &str = r#" class="w-full h-full" p-1
 复杂字段测试 /test "alpha"
       />"#;
    let mut lexer = HtmlLexer {
      _allocator: &Allocator::default(),
      source: Source::new(SOURCE_TEXT),
      state: LexerState::new(LexerStateKind::AfterTagName),
      errors: Vec::new(),
    };

    let tokens: Vec<Token<HtmlKind>> = lexer.tokens().collect();
    insta::assert_snapshot!(format!(
      "Source: {:#?}; \nTokens:{:#?}",
      SOURCE_TEXT, tokens
    ));
  }

  #[test]
  fn after_tag_name_should_return_error() {
    const SOURCE_TEXT: &str = r#" class="w-full"#;
    let mut lexer = HtmlLexer {
      _allocator: &Allocator::default(),
      source: Source::new(SOURCE_TEXT),
      state: LexerState::new(LexerStateKind::AfterTagName),
      errors: Vec::new(),
    };

    lexer.tokens().for_each(drop);

    insta::assert_snapshot!(format!(
      "Source: {:#?}; \nErrors:{:#?}",
      SOURCE_TEXT,
      lexer.errors.first().unwrap()
    ));
  }
}
