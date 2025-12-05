use crate::lexer::state::{LexerState, LexerStateKind};
use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;
use umc_parser::source::Source;

mod kind;
mod lexe;
mod state;

pub(crate) struct HtmlLexer<'a> {
  _allocator: &'a Allocator,
  source: Source<'a>,
  state: LexerState,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> HtmlLexer<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> HtmlLexer<'a> {
    HtmlLexer {
      _allocator: allocator,
      source: Source::new(source_text),
      state: LexerState::new(LexerStateKind::Content),
      errors: Vec::new(),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::{HtmlLexer, kind::HtmlKind};
  use insta::assert_snapshot;
  use oxc_allocator::Allocator;
  use umc_parser::token::Token;

  fn test(source_text: &str) -> String {
    let result: Vec<Token<HtmlKind>> = HtmlLexer::new(&Allocator::default(), source_text)
      .tokens()
      .collect();

    format!("{:#?}", result)
  }

  #[test]
  fn get_tokens() {
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

    assert_snapshot!(test(HTML_STRING));
  }

  #[test]
  fn process_embedded_content() {
    const HTML_STRING: &str = r#"      <!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <script>
    const a = 1;
    const b = 2;
    console.log(a<b);
  </script>
</body>
</html>"#;

    assert_snapshot!(test(HTML_STRING));
  }

  #[test]
  fn self_close_script_tag() {
    const HTML_STRING: &str = r#"      <!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <script />
</body>
</html>"#;

    assert_snapshot!(test(HTML_STRING));
  }
}
