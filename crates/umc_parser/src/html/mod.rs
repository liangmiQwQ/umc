use oxc_parser::ParseOptions;

pub struct HtmlParserOptions {
  // If no parse
  parse_script: Option<ParseOptions>,
}

impl Default for HtmlParserOptions {
  fn default() -> Self {
    HtmlParserOptions {
      parse_script: Some(ParseOptions::default()),
    }
  }
}
