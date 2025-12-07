use oxc_allocator::Allocator;
use umc_html_ast::Node;
use umc_parser::{LanguageParser, ParseResult, ParserImpl};

use crate::{
  Html,
  lexer::{HtmlLexer, HtmlLexerOption},
  option::HtmlParserOption,
};

pub struct HtmlParserImpl<'a> {
  allocator: &'a Allocator,
  source_text: &'a str,
  options: &'a HtmlParserOption,
}

impl<'a> ParserImpl<'a, Html> for HtmlParserImpl<'a> {
  fn new(
    allocator: &'a Allocator,
    source_text: &'a str,
    options: &'a <Html as LanguageParser>::Option,
  ) -> Self {
    HtmlParserImpl {
      allocator,
      source_text,
      options,
    }
  }

  fn parse(self) -> ParseResult<Vec<Node>> {
    let mut lexer = HtmlLexer::new(
      self.allocator,
      self.source_text,
      HtmlLexerOption {
        embedded_language_tags: &self.options.embedded_language_tags,
      },
    );
    let _: Vec<_> = lexer.tokens().collect();
    todo!()
  }
}
