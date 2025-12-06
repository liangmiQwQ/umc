use oxc_allocator::Allocator;
use oxc_parser::ParseOptions;
use umc_html_ast::Node;
use umc_parser::{LanguageParser, ParseResult, Parser, ParserImpl};

use crate::lexer::HtmlLexer;

mod lexer;

pub struct Html;

impl LanguageParser for Html {
  type Ast = Vec<Node>;
  type Option = HtmlOption;
  type Parser = HtmlParser;
}

pub struct HtmlOption {
  /// The oxc_parser options for parsing content inside <script> tags.
  /// If get None, the content in <script> tag will be returned without parsing
  pub parse_script: Option<ParseOptions>,
}

pub struct HtmlParser; // TODO

impl ParserImpl<Html> for HtmlParser {
  fn new(
    allocator: &Allocator,
    source_text: &str,
    options: &<Html as LanguageParser>::Option,
  ) -> Self {
    todo!("{:p}, {:p}, {:p}", &allocator, &source_text, &options)
  }

  fn parse(self) -> ParseResult<Vec<Node>> {
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

pub fn parse<T: LanguageParser>(parser: &Parser<T>, _option: &HtmlOption) {
  let mut lexer = HtmlLexer::new(parser.allocator, parser.source_text);

  let _: Vec<_> = lexer.tokens().collect();
}
