use crate::html5::lexer::source::Source;
use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

mod kind;
mod lexe;
mod source;
mod token;

#[repr(u8)]
pub enum Html5LexerState {
  /// In the element content
  /// e.g. <p>Hello| World<p>
  Content,
  /// After < but before the tag name
  /// e.g. <|a>foo</a>
  InTag,
  /// After tag name but before the tag end
  /// e.g. <a|>foo</a> or <a href|="https://example.com">foo</a>
  AfterTagName,
  /// Finished lexing
  Finished,
}

pub(crate) struct Html5Lexer<'a> {
  allocator: &'a Allocator,
  source: Source<'a>,
  state: Html5LexerState,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> Html5Lexer<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Html5Lexer<'a> {
    Html5Lexer {
      allocator,
      source: Source::new(source_text),
      state: Html5LexerState::Content,
      errors: Vec::new(),
    }
  }
}
