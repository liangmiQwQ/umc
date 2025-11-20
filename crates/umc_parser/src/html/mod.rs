use crate::Parser;
use oxc_parser::ParseOptions;

pub enum HtmlType {
  Auto, // Read the !DOCTYPE tag, use Html5 by default
  XHtml,
  Html5,
}

pub struct HtmlParserOptions {
  // If get None, the content in <script> tag will be returned without parsing
  parse_script: Option<ParseOptions>,
  html_type: HtmlType,
}

impl Default for HtmlParserOptions {
  fn default() -> Self {
    HtmlParserOptions {
      parse_script: Some(ParseOptions::default()),
      html_type: HtmlType::Auto,
    }
  }
}

pub fn parse(parser: &Parser, option: &HtmlParserOptions) {}
