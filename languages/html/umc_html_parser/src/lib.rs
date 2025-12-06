//! HTML parser implementation for the Universal Markup-language Compiler.
//!
//! This crate provides a complete HTML parser that can tokenize and parse HTML
//! documents into an Abstract Syntax Tree (AST). It supports embedded languages
//! like JavaScript (in `<script>` tags) and CSS (in `<style>` tags).
//!
//! # Example
//!
//! ```ignore
//! use umc_html_parser::CreateHtml;
//! use umc_parser::Parser;
//! use oxc_allocator::Allocator;
//!
//! let allocator = Allocator::default();
//! let parser = Parser::html(&allocator, "<html><body>Hello</body></html>");
//! let result = parser.parse();
//! ```

use oxc_allocator::Allocator;
use oxc_parser::ParseOptions;
use std::collections::HashSet;
use umc_html_ast::Node;
use umc_parser::{LanguageParser, ParseResult, Parser, ParserImpl};

use crate::{
  implement::HtmlParserImpl,
  lexer::{HtmlLexer, HtmlLexerOption},
  option::HtmlParserOption,
};

mod lexer;

/// HTML language parser marker type.
///
/// This zero-sized type implements [`LanguageParser`] for HTML parsing.
/// Use [`Parser::html()`](CreateHtml::html) to create an HTML parser instance.
pub struct Html;

impl LanguageParser for Html {
  type Result = Vec<Node>;
  type Option = HtmlParserOption;
  type Parser<'a> = HtmlParserImpl<'a>;
}

/// Convenience trait for creating HTML parsers.
///
/// This trait provides a more ergonomic API for creating HTML parser instances.
///
/// # Example
///
/// ```ignore
/// use umc_parser::Parser;
/// use umc_html_parser::CreateHtml;
/// use oxc_allocator::Allocator;
///
/// let allocator = Allocator::default();
/// let parser = Parser::html(&allocator, "<html></html>");
/// ```
pub trait CreateHtml<'a> {
  /// Create a parser for HTML parsing.
  ///
  /// # Parameters
  /// - `allocator`: Memory arena for allocating AST nodes
  /// - `source_text`: HTML source code to parse
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self;
}

impl<'a> CreateHtml<'a> for Parser<'a, Html> {
  /// Create a parser for Html parsing
  fn html(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Parser::<Html>::new(allocator, source_text)
  }
}

/// HTML parser configuration options.
///
/// This module contains the [`HtmlParserOption`] struct for configuring
/// how the HTML parser handles embedded languages and special content.
pub mod option {
  use super::*;

  /// HTML parser configuration options.
  ///
  /// Configures how the HTML parser handles embedded languages like JavaScript and CSS.
  pub struct HtmlParserOption {
    /// The oxc_parser options for parsing content inside <script> tags.
    /// If get None, the content in <script> tag will be returned without parsing
    pub parse_script: Option<ParseOptions>,
    /// Set of tag names that contain embedded language content (e.g., "script", "style")
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
}

mod implement {
  use super::*;
  use crate::option::HtmlParserOption;

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
}
