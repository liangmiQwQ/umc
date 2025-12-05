use oxc_allocator::Allocator;
use oxc_parser::ParseOptions;
use umc_parser::{Language, Parser, ParserImpl};

use crate::lexer::Html5Lexer;

mod lexer;

pub struct Html5;

impl Language for Html5 {
  type Result = Html5Ast;
  type Option = Html5Option;
  type Parser = Html5Parser;
}

pub struct Html5Option {
  /// The oxc_parser options for parsing content inside <script> tags.
  /// If get None, the content in <script> tag will be returned without parsing
  pub parse_script: Option<ParseOptions>,
}

pub struct Html5Ast; // TODO

pub struct Html5Parser; // TODO

impl ParserImpl<Html5> for Html5Parser {
  fn new(allocator: &Allocator, source_text: &str, options: &<Html5 as Language>::Option) -> Self {
    todo!()
  }

  fn parse(self) -> <Html5 as Language>::Result {
    todo!()
  }
}

impl Default for Html5Option {
  fn default() -> Self {
    Html5Option {
      parse_script: Some(ParseOptions::default()),
    }
  }
}

pub trait CreateHtml<'a> {
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self;
}

impl<'a> CreateHtml<'a> for Parser<'a, Html5> {
  /// Create a parser for Html5 parsing
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Parser::<Html5>::new(allocator, source_text)
  }
}

pub fn parse<T: Language>(parser: &Parser<T>, _option: &Html5Option) {
  let mut lexer = Html5Lexer::new(parser.allocator, parser.source_text);

  let _: Vec<_> = lexer.tokens().collect();
}
