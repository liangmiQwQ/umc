use crate::Parser;
use oxc_parser::ParseOptions;

pub struct HtmlParserOptions {
  // If get None, the content in <script> tag will be returned without parsing
  parse_script: Option<ParseOptions>,
}

impl Default for HtmlParserOptions {
  fn default() -> Self {
    HtmlParserOptions {
      parse_script: Some(ParseOptions::default()),
    }
  }
}

pub fn parse(parser: &Parser, option: &HtmlParserOptions) {}
