use oxc_parser::ParseOptions;

pub struct HtmlParserOptions {
  parse_script: Option<ParseOptions>,
}

impl Default for HtmlParserOptions {
  fn default() -> Self {
    HtmlParserOptions {
      parse_script: Some(ParseOptions::default()),
    }
  }
}
