use crate::html5::lexer::source::Source;
use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

mod kind;
mod lexe;
mod source;
mod token;

#[repr(u8)]
pub enum Html5LexerState<'a> {
  /// In the element content
  /// e.g. <p>Hello| World<p>
  Content,
  /// Don't treat < as tag end unless it's followed by the tag end
  /// The parameter is the tag end, e.g. </script
  EmbeddedContent(&'a str),
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
  _allocator: &'a Allocator,
  source: Source<'a>,
  state: Html5LexerState<'a>,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> Html5Lexer<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Html5Lexer<'a> {
    Html5Lexer {
      _allocator: allocator,
      source: Source::new(source_text),
      state: Html5LexerState::Content,
      errors: Vec::new(),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::html5::lexer::{
    Html5Lexer, kind::Html5Kind, token::Html5Token, token::Html5TokenValue,
  };
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

    insta::assert_snapshot!(format!("{:#?}", result))
  }
}
