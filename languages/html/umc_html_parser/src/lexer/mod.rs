use crate::lexer::{
  source::Source,
  state::{LexerState, LexerStateKind},
};
use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

mod kind;
mod lexe;
mod source;
mod state;
mod token;

pub(crate) struct Html5Lexer<'a> {
  _allocator: &'a Allocator,
  source: Source<'a>,
  state: LexerState,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> Html5Lexer<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Html5Lexer<'a> {
    Html5Lexer {
      _allocator: allocator,
      source: Source::new(source_text),
      state: LexerState::new(LexerStateKind::Content),
      errors: Vec::new(),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::html5::lexer::{Html5Lexer, token::Html5Token};
  use insta::assert_snapshot;
  use oxc_allocator::Allocator;

  fn test(source_text: &str) -> String {
    let result: Vec<Html5Token> = Html5Lexer::new(&Allocator::default(), source_text)
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
