use oxc_allocator::Allocator;
use oxc_parser::ParseOptions;
use umc_parser::{Language, Parser, ParserImpl};

use crate::lexer::HtmlLexer;

mod lexer;

pub struct Html;

impl Language for Html {
  type Result = HtmlAst;
  type Option = HtmlOption;
  type Parser = HtmlParser;
}

pub struct HtmlOption {
  /// The oxc_parser options for parsing content inside <script> tags.
  /// If get None, the content in <script> tag will be returned without parsing
  pub parse_script: Option<ParseOptions>,
}

pub struct HtmlAst; // TODO

pub struct HtmlParser; // TODO

impl ParserImpl<Html> for HtmlParser {
  fn new(allocator: &Allocator, source_text: &str, options: &<Html as Language>::Option) -> Self {
    todo!("{:p}, {:p}, {:p}", &allocator, &source_text, &options)
  }

  fn parse(self) -> <Html as Language>::Result {
    todo!()
  }
}

impl Default for HtmlOption {
  fn default() -> Self {
    HtmlOption {
      parse_script: Some(ParseOptions::default()),
    }
  }
}

pub trait CreateHtml<'a> {
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self;
}

impl<'a> CreateHtml<'a> for Parser<'a, Html> {
  /// Create a parser for Html parsing
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Parser::<Html>::new(allocator, source_text)
  }
}

pub fn parse<T: Language>(parser: &Parser<T>, _option: &HtmlOption) {
  let mut lexer = HtmlLexer::new(parser.allocator, parser.source_text);

  let _: Vec<_> = lexer.tokens().collect();
}
