use crate::lexer::state::{LexerState, LexerStateKind};
use oxc_diagnostics::OxcDiagnostic;
use umc_parser::source::Source;

pub(crate) mod kind;
mod lexe;
mod state;

pub(crate) struct HtmlLexerOption<'a> {
  pub is_embedded_language_tag: &'a dyn Fn(&str) -> bool,
}

pub(crate) struct HtmlLexer<'a> {
  source: Source<'a>,
  state: LexerState,
  option: HtmlLexerOption<'a>,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> HtmlLexer<'a> {
  pub fn new(source_text: &'a str, option: HtmlLexerOption<'a>) -> HtmlLexer<'a> {
    HtmlLexer {
      source: Source::new(source_text),
      state: LexerState::new(LexerStateKind::Content),
      option,
      errors: Vec::new(),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::{HtmlLexer, HtmlLexerOption, kind::HtmlKind};
  use insta::assert_snapshot;
  use umc_parser::token::Token;

  fn test(source_text: &str) -> String {
    let func =
      |tag_name: &str| matches!(tag_name.to_ascii_lowercase().as_str(), "script" | "style");

    let mut lexer = HtmlLexer::new(
      source_text,
      HtmlLexerOption {
        is_embedded_language_tag: &func,
      },
    );

    let result: Vec<Token<HtmlKind>> = lexer.tokens().collect();

    format!("Tokens: {:#?}\nErrors: {:#?}", result, lexer.errors)
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
    const HTML_STRING: &str = r#"
<html lang="en">
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
    const HTML_STRING: &str = r#"
<html lang="en">
<body>
  <script />
</body>
</html>"#;

    assert_snapshot!(test(HTML_STRING));
  }

  // errors
  #[test]
  fn no_complete_doctype() {
    const HTML_STRING: &str = r#"<!DOCTYP"#;

    assert_snapshot!(test(HTML_STRING));
  }

  #[test]
  fn no_complete_comment() {
    const HTML_STRING: &str = r#"<!-"#;

    assert_snapshot!(test(HTML_STRING));
  }

  #[test]
  fn no_string_end() {
    const HTML_STRING: &str = r#"<p href="https://www.google.com"#;

    assert_snapshot!(test(HTML_STRING));
  }

  #[test]
  fn no_closing_tag_for_embedded_content() {
    const HTML_STRING: &str = "<script> const a = 1; ";

    assert_snapshot!(test(HTML_STRING));
  }
}
