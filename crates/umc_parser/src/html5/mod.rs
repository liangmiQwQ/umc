use crate::{Parser, html5::lexer::Html5Lexer};
use oxc_parser::ParseOptions;

mod lexer;

pub struct Html5ParserOptions {
  /// The oxc_parser options for parsing content inside <script> tags.
  /// If get None, the content in <script> tag will be returned without parsing
  parse_script: Option<ParseOptions>,
}

impl Default for Html5ParserOptions {
  fn default() -> Self {
    Html5ParserOptions {
      parse_script: Some(ParseOptions::default()),
    }
  }
}

pub fn parse(parser: &Parser, option: &Html5ParserOptions) {
  let mut lexer = Html5Lexer::new(parser.allocator, parser.source_text);

  let _: Vec<_> = lexer.tokens().collect();
}
