use memchr::{memchr, memchr_iter, memmem::find};
use oxc_diagnostics::OxcDiagnostic;
use std::iter::from_fn;
use umc_parser::token::Token;
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
    println!("{}", str::from_utf8(self.source.rest()).unwrap());

    let start = self.source.pointer;

    match self.source.get(start).unwrap() {
      b'<' => {
        match self.source.get(start + 1) {
          Some(item) if item.is_ascii_alphabetic() => {
            self.source.advance(1);
            let result = Token::<HtmlKind> {
              kind: HtmlKind::TagStart,
              start,
              end: self.source.pointer,
            };

            self.state.kind = LexerStateKind::InTag; // update state
            self.state.allow_to_set_tag_name();
            result
          }

          // for / character, as closing tag
          Some(b'/') => {
            self.source.advance(2);
            let result = Token::<HtmlKind> {
              kind: HtmlKind::CloseTagStart,
              start,
              end: self.source.pointer,
            };

            self.state.kind = LexerStateKind::InTag; // update state
            result
          }

          // for ! character, as comment or doctype
          Some(b'!') => {
            const DOCTYPE: &[u8] = b"doctype";
            const COMMENT_START: &[u8] = b"!--";

            self.source.advance(2);
            if self.source.starts_with_lowercase(DOCTYPE) {
              self.source.advance(DOCTYPE.len() as u32);
              let result = Token::<HtmlKind> {
                kind: HtmlKind::Doctype,
                start,
                end: self.source.pointer,
              };

              self.state.kind = LexerStateKind::AfterTagName;
              result
            } else if self.source.starts_with_lowercase(COMMENT_START) {
              // for <!--> situation, don't go ahead
              let comment_end = find(self.source.rest(), b"-->");

              if let Some(end) = comment_end.map(|i| i as u32) {
                self.source.advance(end + 3);
                Token::<HtmlKind> {
                  kind: HtmlKind::Comment,
                  start,
                  end: self.source.pointer,
                }
              } else {
                self.tailless_comment(start)
              }
            } else {
              let comment_end = memchr(b'>', self.source.rest());

              if let Some(end) = comment_end.map(|i| i as u32) {
                self.source.advance(end + 1);
                Token::<HtmlKind> {
                  kind: HtmlKind::Comment,
                  start,
                  end: self.source.pointer,
                }
              } else {
                self.tailless_comment(start)
              }
            }
          }

          Some(_) | None => self.handle_content_text(),
        }
      }
      _ => self.handle_content_text(),
    }
  }

  fn handle_content_text(&mut self) -> Token<HtmlKind> {
    let mut index = self.source.source_text.len() as u32;
    let mut iter = memchr_iter(b'<', self.source.rest());

    while let Some(i) = iter.next().map(|i| i as u32) {
      if let Some(next) = self.source.get(i + 1)
        && (next.is_ascii_alphabetic() || next == b'/' || next == b'!')
      {
        index = i + self.source.pointer;
        break;
      }
    }

    let start = self.source.pointer;
    self.source.to(index);

    Token::<HtmlKind> {
      kind: HtmlKind::TextContent,
      start,
      end: self.source.pointer,
    }
  }

  fn tailless_comment(&mut self, start: u32) -> Token<HtmlKind> {
    // eof without finishing doctype or comment
    self.source.to(self.source.source_text.len() as u32);

    // throw an error
    self.errors.push(
      OxcDiagnostic::error(format!(
        "Expected {}, but found {}",
        HtmlKind::TagEnd,
        HtmlKind::Eof
      ))
      .with_label(Span::new(self.source.pointer, self.source.pointer)),
    );

    // return as comment
    Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start,
      end: self.source.pointer,
    }
  }
}

// handler for HtmlLexerState::EmbeddedContent
impl<'a> HtmlLexer<'a> {
  fn handle_embedded_content(&mut self) -> Token<HtmlKind> {
    let closing_tag_string = format!("</{}", self.state.take_tag_name().unwrap());
    let closing_tag = closing_tag_string.as_bytes(); // safe unwrap because only script/style can enter this state

    let start = self.source.pointer;
    let mut end = self.source.source_text.len() as u32;

    if let Some(tag_end) = find(self.source.rest(), closing_tag).map(|e| e as u32) {
      end = start + tag_end;
      self.state.kind = LexerStateKind::Content; // update state
    } else {
      self.errors.push(
        OxcDiagnostic::error(format!(
          "Expected {}, but found {}",
          str::from_utf8(closing_tag).unwrap(),
          HtmlKind::Eof
        ))
        .with_label(Span::new(end, end)),
      );
    }

    self.source.to(end);

    Token::<HtmlKind> {
      kind: HtmlKind::TextContent,
      start,
      end: self.source.pointer,
    }
  }
}

// handler for HtmlLexerState::AfterTagName
impl<'a> HtmlLexer<'a> {
  fn handle_after_tag_name(&mut self) -> Token<HtmlKind> {
    let start = self.source.pointer;

    // safe unwarp, won't direct to this branch if pointer == file.len()
    match self.source.get(start).unwrap() {
      w if w.is_ascii_whitespace() => {
        self.source.advance(1);
        let mut i = 0;
        while i < self.source.rest().len() && self.source.rest()[i].is_ascii_whitespace() {
          i += 1;
        }

        self.source.advance(i as u32);

        Token::<HtmlKind> {
          kind: HtmlKind::Whitespace,
          start,
          end: self.source.pointer,
        }
      }

      b'=' => {
        self.source.advance(1);

        Token::<HtmlKind> {
          kind: HtmlKind::Eq,
          start,
          end: self.source.pointer,
        }
      }

      b'>' => {
        self.source.advance(1);

        if let Some(tag_name) = self.state.get_tag_name()
          && (self.option.is_embedded_language_tag)(tag_name)
        {
          self.state.kind = LexerStateKind::EmbeddedContent;
        } else {
          self.state.kind = LexerStateKind::Content;
        }

        Token::<HtmlKind> {
          kind: HtmlKind::TagEnd,
          start,
          end: self.source.pointer,
        }
      }

      b'/' => {
        if let Some(next) = self.source.get(self.source.pointer + 1)
          && next == b'>'
        {
          self.source.advance(2);
          Token::<HtmlKind> {
            kind: HtmlKind::SelfCloseTagEnd,
            start,
            end: self.source.pointer,
          }
        } else {
          self.handle_tag(start, HtmlKind::Attribute)
        }
      }

      // for attribute with `"`
      b'"' => {
        self.source.advance(1);
        self.handle_quote_attribute(start, b'"')
      }

      // for attribute with `'`
      b'\'' => {
        self.source.advance(1);
        self.handle_quote_attribute(start, b'\'')
      }

      // for attribute without `"`
      _ => self.handle_tag(start, HtmlKind::Attribute),
    }
  }

  fn handle_quote_attribute(&mut self, start: u32, quote: u8) -> Token<HtmlKind> {
    // since html don't support \ escape, we don't need to manage its state
    let mut end = self.source.source_text.len() as u32;

    if let Some(index) = memchr(quote, self.source.rest()) {
      end = self.source.pointer + index as u32;
    } else {
      // throw an error, expect quote, but found eof
      self.errors.push(
        OxcDiagnostic::error(format!(
          "Expected {}, but found {}",
          char::from(quote),
          HtmlKind::Eof
        ))
        .with_label(Span::new(end, end)),
      );
    }

    self.source.to(end);

    Token::<HtmlKind> {
      kind: HtmlKind::Attribute,
      start,
      end,
    }
  }
}

// handler for HtmlLexerState::InTag
impl<'a> HtmlLexer<'a> {
  fn handle_in_tag(&mut self) -> Token<HtmlKind> {
    // call the handle_tag
    let result = self.handle_tag(self.source.pointer, HtmlKind::ElementName);
    self.state.kind = LexerStateKind::AfterTagName; // update state
    self.state.set_tag_name(
      str::from_utf8(&self.source.source_text[result.start as usize..result.end as usize]).unwrap(),
    );
    result
  }
}

// some universal functions
impl<'a> HtmlLexer<'a> {
  fn handle_tag(&mut self, start: u32, kind: HtmlKind) -> Token<HtmlKind> {
    let mut i = 0;
    while i < self.source.rest().len()
      && !{
        let item = self.source.rest()[i];
        item.is_ascii_whitespace() || item == b'>' || item == b'=' || item == b'/'
      }
    {
      i += 1;
    }

    self.source.advance(i as u32);

    Token::<HtmlKind> {
      kind,
      start,
      end: self.source.pointer,
    }
  }
}
