//! Core parser infrastructure for the Universal Markup-language Compiler.
//!
//! This crate provides the foundational traits and types for implementing
//! language-specific parsers. It defines a generic parser framework that can
//! be used to parse different markup languages (HTML, XML, etc.).
//!
//! # Example
//!
//! ```ignore
//! use umc_parser::{Parser, LanguageParser};
//! use oxc_allocator::Allocator;
//!
//! // Assuming Html implements LanguageParser
//! let allocator = Allocator::default();
//! let parser = Parser::<Html>::new(&allocator, "<html></html>");
//! let result = parser.parse();
//! ```

use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

/// Source text tracking and navigation.
pub mod source;
/// Token types and utilities.
pub mod token;

/// Core trait for implementing language-specific parsers.
///
/// This trait defines the contract for creating parsers for different markup languages.
/// Each language implementation must specify its result type, options, and parser implementation.
///
/// The `Result` type uses a Generic Associated Type (GAT) with lifetime `'a` to support
/// arena-allocated AST nodes. This allows the parsed result to reference data in the
/// allocator's memory arena.
///
/// # Example
///
/// ```ignore
/// struct Html;
///
/// impl LanguageParser for Html {
///   type Result<'a> = Program<'a>;
///   type Option = HtmlParserOption;
///   type Parser<'a> = HtmlParserImpl<'a>;
/// }
/// ```
pub trait LanguageParser: Sized {
  /// The type of the parsed result (e.g., AST root node or node collection).
  /// Uses a lifetime parameter to support arena-allocated data.
  type Result<'a>;
  /// Parser configuration options, must have a default implementation
  type Option: Default;
  /// The concrete parser implementation for this language
  type Parser<'a>: ParserImpl<'a, Self>;
}

/// Implementation trait for language-specific parser instances.
///
/// This trait defines the methods that must be implemented by concrete parser types.
/// It handles the actual parsing logic for a specific language.
pub trait ParserImpl<'a, T: LanguageParser> {
  /// Create a new parser instance.
  ///
  /// # Parameters
  /// - `allocator`: Memory arena for allocating AST nodes
  /// - `source_text`: Source code to parse
  /// - `options`: Language-specific parser options
  fn new(allocator: &'a Allocator, source_text: &'a str, options: &'a T::Option) -> Self;

  /// Parse the source text and return the result.
  ///
  /// Consumes the parser and returns a [`ParseResult`] containing the parsed program
  /// and any errors encountered during parsing.
  fn parse(self) -> ParseResult<T::Result<'a>>;
}

/// Generic parser wrapper for any language implementing [`LanguageParser`].
///
/// This struct provides a convenient API for creating and configuring parsers
/// for different markup languages.
pub struct Parser<'a, T: LanguageParser> {
  /// Memory arena for allocating AST nodes during parsing
  pub allocator: &'a Allocator,
  /// The source code text to be parsed
  pub source_text: &'a str,
  /// Language-specific parser configuration options
  pub options: T::Option,
}

/// Result of a parsing operation.
///
/// Contains both the parsed program/AST and any diagnostic errors encountered
/// during parsing. Errors are non-fatal and the parser attempts to recover and
/// continue parsing.
pub struct ParseResult<T> {
  /// The parsed program or AST root
  pub program: T,
  /// Diagnostic errors encountered during parsing
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a, T: LanguageParser> Parser<'a, T> {
  /// Create the umc parser
  ///
  /// # Parameters
  /// - `allocator`: [Memory arena](oxc_allocator::Allocator) for allocating AST nodes
  /// - `source_text`: Source code to parse
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Self {
    Self {
      allocator,
      source_text,
      options: T::Option::default(),
    }
  }

  /// Override the parser option
  #[must_use]
  pub fn with_options(mut self, options: T::Option) -> Self {
    self.options = options;
    self
  }

  /// Get the parse result.
  ///
  /// Takes `&'a self` to ensure the options reference has the same lifetime
  /// as the allocator and source text, which is required for arena allocation.
  pub fn parse(&'a self) -> ParseResult<T::Result<'a>> {
    let parser = T::Parser::new(self.allocator, self.source_text, &self.options);

    parser.parse()
  }
}
