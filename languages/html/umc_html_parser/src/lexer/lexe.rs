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
    if self.source.is_eof() {
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
  fn finish(&mut self) -> Token<HtmlKind> {
    self.state.kind = LexerStateKind::Finished; // mark as finished

    Token::<HtmlKind> {
      kind: HtmlKind::Eof,
      start: self.source.position,
      end: self.source.position,
    }
  }
}

// handler for HtmlLexerState::Content
impl<'a> HtmlLexer<'a> {
  fn handle_content(&mut self) -> Token<HtmlKind> {
    let start = self.source.position;
    // safe unwrap, won't direct to this branch if EOF
    match self.source.bump().unwrap() {
      // for <
      '<' => {
        // maybe comment, doctype, tag or < starting content
        match self.source.peek() {
          // for alphabetic character, as tag start
          Some(&item) if item.is_alphabetic() => {
            // do not consume the alphabetic char, TagStart is just '<'
            let result = Token::<HtmlKind> {
              kind: HtmlKind::TagStart,
              start,
              end: self.source.position,
            };

            self.state.kind = LexerStateKind::InTag; // update state
            self.state.allow_to_set_tag_name();
            result
          }

          // for / character, as closing tag
          Some(&'/') => {
            self.source.bump(); // consume '/'

            let result = Token::<HtmlKind> {
              kind: HtmlKind::CloseTagStart,
              start,
              end: self.source.position,
            };

            self.state.kind = LexerStateKind::InTag; // update state
            result
          }

          // for ! character, as comment or doctype
          Some(&'!') => {
            self.source.bump(); // consume '!'

            const COMMENT_START: [char; 2] = ['-', '-'];
            const DOCTYPE_START: [char; 7] = ['D', 'O', 'C', 'T', 'Y', 'P', 'E'];
            let mut match_doctype = true;
            let mut match_comment = true;
            let mut i = 0;

            while let Some(&item) = self.source.peek() {
              self.source.bump(); // consume the peeked char

              if match_doctype
                && matches!(DOCTYPE_START.get(i), Some(&c) if c.eq_ignore_ascii_case(&item))
              {
                if i == DOCTYPE_START.len() - 1 {
                  // it's a doctype
                  let result = Token::<HtmlKind> {
                    kind: HtmlKind::Doctype,
                    start,
                    end: self.source.position,
                  };

                  self.state.kind = LexerStateKind::AfterTagName; // update state
                  return result;
                }
              } else {
                match_doctype = false;
              }

              if match_comment && COMMENT_START.get(i) == Some(&item) {
                if i == COMMENT_START.len() {
                  // it's a comment
                  return self.handle_comment(start);
                }
              } else {
                match_comment = false;
              }

              if !match_doctype && !match_comment {
                // it is neither doctype nor comment, treat as bogus comment (ends with > instead of -->)
                return self.handle_bogus_comment(start);
              }

              i += 1
            }

            self.tailless_comment(start)
          }

          // for none and other character, as content starting with <
          None | Some(_) => {
            // record until next tag start
            self.handle_content_text(start)
          }
        }
      }

      // for content
      _ => {
        // record until next tag start
        self.handle_content_text(start)
      }
    }
  }

  fn handle_bogus_comment(&mut self, start: u32) -> Token<HtmlKind> {
    let mut ended = false;
    while let Some(item) = self.source.bump() {
      if item == '>' {
        ended = true;
        break;
      }
    }

    if !ended {
      // eof without finishing doctype or comment
      return self.tailless_comment(start);
    }

    Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start,
      end: self.source.position,
    }
    // It still on Content state like this: sometest|<! something >| moretext
  }

  fn handle_comment(&mut self, start: u32) -> Token<HtmlKind> {
    let mut dash_count: u8 = 0;
    let mut ended = false;

    while let Some(item) = self.source.bump() {
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
      return self.tailless_comment(start);
    }

    Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start,
      end: self.source.position,
    }
    // It still on Content state like this: sometest|<!-- something -->| moretext
  }

  fn tailless_comment(&mut self, start: u32) -> Token<HtmlKind> {
    // eof without finishing doctype or comment
    // throw an error
    self.errors.push(
      OxcDiagnostic::error(format!(
        "Expected {}, but found {}",
        HtmlKind::TagEnd,
        HtmlKind::Eof
      ))
      .with_label(Span::new(self.source.position, self.source.position)),
    );

    // return as comment
    let result = Token::<HtmlKind> {
      kind: HtmlKind::Comment,
      start,
      end: self.source.position,
    };

    self.state.kind = LexerStateKind::AfterTagName; // update state
    result
  }

  fn handle_content_text(&mut self, start: u32) -> Token<HtmlKind> {
    loop {
      let remaining = self.source.remaining();

      if remaining.is_empty() {
        break;
      }

      if let Some(index) = remaining.find('<') {
        if index > 0 {
          self.source.advance(index as u32);
        }

        // Check if it is a real tag start
        // We are strictly at '<' now
        let remaining_after = self.source.remaining();
        let mut chars = remaining_after[1..].chars();

        if let Some(c) = chars.next() {
          if c.is_alphabetic() || c == '/' || c == '!' {
            break;
          } else {
            self.source.bump();
          }
        } else {
          self.source.bump();
        }
      } else {
        self.source.advance(remaining.len() as u32);
        break;
      }
    }

    Token::<HtmlKind> {
      kind: HtmlKind::TextContent,
      start,
      end: self.source.position,
    }
  }
}

// handler for HtmlLexerState::EmbeddedContent
impl<'a> HtmlLexer<'a> {
  fn handle_embedded_content(&mut self) -> Token<HtmlKind> {
    let start = self.source.position;
    let closing_tag = format!("</{}", self.state.take_tag_name().unwrap()); // safe unwrap because only script/style can enter this state
    let mut ended = false;

    while self.source.bump().is_some() {
      if self.source.source_text[(self.source.position) as usize..].starts_with(&closing_tag) {
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
        .with_label(Span::new(self.source.position, self.source.position)),
      );
    }

    let result = Token::<HtmlKind> {
      start,
      end: self.source.position,
      kind: HtmlKind::TextContent,
    };
    self.state.kind = LexerStateKind::Content; // update state
    result
  }
}

// handler for HtmlLexerState::AfterTagName
impl<'a> HtmlLexer<'a> {
  fn handle_after_tag_name(&mut self) -> Token<HtmlKind> {
    let start = self.source.position;
    // safe unwrap, won't direct to this branch if EOF
    match self.source.bump().unwrap() {
      // for whitespace
      c if c.is_whitespace() => {
        while let Some(&item) = self.source.peek() {
          if item.is_whitespace() {
            self.source.bump();
          } else {
            break;
          }
        }

        Token::<HtmlKind> {
          start,
          end: self.source.position,
          kind: HtmlKind::Whitespace,
        }
      }

      // for =
      '=' => Token::<HtmlKind> {
        kind: HtmlKind::Eq,
        start,
        end: self.source.position,
      },

      // for tag end (>)
      '>' => {
        let result = Token::<HtmlKind> {
          kind: HtmlKind::TagEnd,
          start,
          end: self.source.position,
        };

        // update state
        if let Some(tag_name) = self.state.get_tag_name()
          && (self.option.is_embedded_language_tag)(tag_name)
        {
          self.state.kind = LexerStateKind::EmbeddedContent;
        } else {
          self.state.kind = LexerStateKind::Content;
        }

        result
      }

      // for self close end and attribute starts with `/`
      '/' => {
        match self.source.peek() {
          Some(&'>') => {
            self.source.bump(); // consume '>'
            // self close
            let result = Token::<HtmlKind> {
              kind: HtmlKind::SelfCloseTagEnd,
              start,
              end: self.source.position,
            };

            self.state.take_tag_name(); // clear tag name
            self.state.kind = LexerStateKind::Content; // update state
            result
          }
          None | Some(_) => self.handle_tag(start, HtmlKind::Attribute),
        }
      }

      // for attribute with `"`
      '"' => self.handle_quote_attribute(start, '"'),

      // for attribute with `'`
      '\'' => self.handle_quote_attribute(start, '\''),

      // for attribute without `"`
      _ => self.handle_tag(start, HtmlKind::Attribute),
    }
  }

  fn handle_quote_attribute(&mut self, start: u32, quote: char) -> Token<HtmlKind> {
    // since html don't support \ escape, we don't need to manage its state
    let mut ended = false;

    while let Some(item) = self.source.bump() {
      if item == quote {
        ended = true;
        break;
      }
    }

    if !ended {
      // throw an error, expect quote, but found eof
      self.errors.push(
        OxcDiagnostic::error(format!("Expected {}, but found {}", quote, HtmlKind::Eof))
          .with_label(Span::new(self.source.position, self.source.position)),
      );
    }

    Token::<HtmlKind> {
      start,
      end: self.source.position,
      kind: HtmlKind::Attribute,
    }
  }
}

// handler for HtmlLexerState::InTag
impl<'a> HtmlLexer<'a> {
  fn handle_in_tag(&mut self) -> Token<HtmlKind> {
    let start = self.source.position;
    let result = self.handle_tag(start, HtmlKind::ElementName);
    self.state.kind = LexerStateKind::AfterTagName; // update state
    self
      .state
      .set_tag_name(self.source.source_text[result.start as usize..result.end as usize].to_owned());
    result
  }
}

// some universal functions
impl<'a> HtmlLexer<'a> {
  fn handle_tag(&mut self, start: u32, kind: HtmlKind) -> Token<HtmlKind> {
    while let Some(&item) = self.source.peek() {
      if item.is_whitespace() || item == '>' || item == '=' || item == '/' {
        // end of a attribute
        break;
      } else {
        self.source.bump();
      }
    }

    Token::<HtmlKind> {
      start,
      end: self.source.position,
      kind,
    }
  }
}
