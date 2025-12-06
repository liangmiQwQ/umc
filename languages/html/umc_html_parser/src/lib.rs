use oxc_allocator::Allocator;
use oxc_parser::ParseOptions;
use std::collections::HashSet;
use umc_html_ast::Node;
use umc_parser::{LanguageParser, ParseResult, Parser, ParserImpl};

use crate::lexer::{HtmlLexer, HtmlLexerOption};

mod lexer;

pub struct Html;

impl LanguageParser for Html {
  type Result = Vec<Node>;
  type Option = HtmlParserOption;
  type Parser<'a> = HtmlParserImpl<'a>;
}

pub struct HtmlParserOption {
  /// The oxc_parser options for parsing content inside <script> tags.
  /// If get None, the content in <script> tag will be returned without parsing
  pub parse_script: Option<ParseOptions>,
  pub embedded_language_tags: HashSet<String>,
}

impl Default for HtmlParserOption {
  fn default() -> Self {
    HtmlParserOption {
      parse_script: Some(ParseOptions::default()),
      embedded_language_tags: {
        let mut set: HashSet<String> = HashSet::new();
        set.insert("script".to_string());
        set.insert("style".to_string());
        set
      },
    }
  }
}

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

pub trait CreateHtml<'a> {
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self;
}

impl<'a> CreateHtml<'a> for Parser<'a, Html> {
  /// Create a parser for Html parsing
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Parser::<Html>::new(allocator, source_text)
  }
}
