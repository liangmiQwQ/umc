#[repr(u8)]
pub(super) enum LexerStateKind {
  /// In the element content
  /// e.g. <p>Hello| World<p>
  Content,
  /// Don't treat < as tag end unless it's followed by the tag end
  /// The parameter is the tag end, e.g. </script
  EmbeddedContent,
  /// After < but before the tag name
  /// e.g. <|a>foo</a>
  InTag,
  /// After tag name but before the tag end
  /// e.g. <a|>foo</a> or <a href|="https://example.com">foo</a>
  AfterTagName,
  /// Finished lexing
  Finished,
}

pub(super) struct LexerState<'a> {
  pub kind: LexerStateKind,
  tag_name: Option<&'a str>,
  allow_to_set_tag_name: bool,
}

impl<'a> LexerState<'a> {
  pub fn new(kind: LexerStateKind) -> Self {
    LexerState {
      kind,
      tag_name: None,
      allow_to_set_tag_name: false,
    }
  }
}

impl<'a> LexerState<'a> {
  pub fn allow_to_set_tag_name(&mut self) {
    self.allow_to_set_tag_name = true;
  }

  pub fn set_tag_name(&mut self, tag_name: &'a str) {
    if self.allow_to_set_tag_name {
      self.tag_name = Some(tag_name);
    }

    self.allow_to_set_tag_name = false;
  }

  pub fn get_tag_name(&mut self) -> Option<&str> {
    self.tag_name
  }

  pub fn take_tag_name(&mut self) -> Option<&str> {
    self.tag_name.take()
  }
}
