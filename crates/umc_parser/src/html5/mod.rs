use crate::{Language, Parser, html5::lexer::Html5Lexer};
use oxc_allocator::Allocator;
use oxc_parser::ParseOptions;

mod lexer;

pub struct Html5;

impl Language for Html5 {
  type Result = Html5Ast;
  type Option = Html5Option;
}

pub struct Html5Option {
  /// The oxc_parser options for parsing content inside <script> tags.
  /// If get None, the content in <script> tag will be returned without parsing
  pub parse_script: Option<ParseOptions>,
}

pub struct Html5Ast {/* TODO */}

impl Default for Html5Option {
  fn default() -> Self {
    Html5Option {
      parse_script: Some(ParseOptions::default()),
    }
  }
}

impl<'a> Parser<'a, Html5> {
  /// Create a parser for Html5 parsing
  pub fn html5(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Parser::<Html5>::new(allocator, source_text)
  }
}

pub fn parse<T: Language>(parser: &Parser<T>, _option: &Html5Option) {
  let mut lexer = Html5Lexer::new(parser.allocator, parser.source_text);

  let _: Vec<_> = lexer.tokens().collect();
}
