use crate::Parser;
use oxc_parser::ParseOptions;

mod lexer;

pub struct Html5ParserOptions {
  // If get None, the content in <script> tag will be returned without parsing
  parse_script: Option<ParseOptions>,
}

impl Default for Html5ParserOptions {
  fn default() -> Self {
    Html5ParserOptions {
      parse_script: Some(ParseOptions::default()),
    }
  }
}

pub fn parse(parser: &Parser, option: &Html5ParserOptions) {}
