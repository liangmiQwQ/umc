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

use oxc_allocator::{Allocator, Vec};
use oxc_parser::ParseOptions;
use umc_html_ast::Node;
use umc_parser::{LanguageParser, Parser};

use crate::{option::HtmlParserOption, parse::HtmlParserImpl};

mod lexer;
mod parse;

/// HTML language parser marker type.
///
/// This zero-sized type implements [`LanguageParser`] for HTML parsing.
/// Use [`Parser::html()`](CreateHtml::html) to create an HTML parser instance.
pub struct Html;

impl LanguageParser for Html {
  /// The parsed result is an arena-allocated vector of AST nodes.
  /// Uses `oxc_allocator::Vec` for cache-friendly traversal and bulk deallocation.
  type Result<'a> = Vec<'a, Node<'a>>;
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
    /// If get None, the content in <script> tag will be regared as [Text](umc_html_ast::Text)
    pub parse_script: Option<ParseOptions>,
    /// A function that returns true if the given tag name is an embedded language tag (e.g., "script", "style")
    ///
    /// # Examples
    /// ```ignore
    /// let option = HtmlParserOption {
    ///   is_embedded_language_tag: Box::new(|tag_name: &str| matches!(tag_name, "script" | "style")),
    ///   // some other options
    /// }
    /// ```
    pub is_embedded_language_tag: Box<dyn Fn(&str) -> bool>,
    /// A function that returns true if the given tag name is a void tag (e.g., "br", "hr", "img")
    ///
    /// # Examples
    /// ```ignore
    /// let option = HtmlParserOption {
    ///   is_void_tag: Box::new(|tag_name: &str| matches!(tag_name, "br" | "hr" | "img")),
    ///   // some other options
    /// }
    /// ```
    pub is_void_tag: Box<dyn Fn(&str) -> bool>,
  }

  impl Default for HtmlParserOption {
    fn default() -> Self {
      HtmlParserOption {
        parse_script: Some(ParseOptions::default()),
        is_embedded_language_tag: Box::new(|tag_name: &str| {
          matches!(tag_name.to_ascii_lowercase().as_str(), "script" | "style")
        }),
        is_void_tag: Box::new(|tag_name: &str| {
          matches!(
            tag_name.to_ascii_lowercase().as_str(),
            "area"
              | "base"
              | "br"
              | "col"
              | "embed"
              | "hr"
              | "img"
              | "input"
              | "keygen"
              | "link"
              | "meta"
              | "param"
              | "source"
              | "track"
              | "wbr"
          )
        }),
      }
    }
  }
}
