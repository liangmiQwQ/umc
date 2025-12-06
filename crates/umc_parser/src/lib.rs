use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

pub mod char;
pub mod source;
pub mod token;

pub trait LanguageParser: Sized {
  type Result;
  type Option: Default;
  type Parser<'a>: ParserImpl<'a, Self>;
}

pub trait ParserImpl<'a, T: LanguageParser> {
  fn new(allocator: &'a Allocator, source_text: &'a str, options: &'a T::Option) -> Self;
  fn parse(self) -> ParseResult<T::Result>;
}

pub struct Parser<'a, T: LanguageParser> {
  pub allocator: &'a Allocator,
  pub source_text: &'a str,
  pub options: T::Option,
}

pub struct ParseResult<T> {
  pub program: T,
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
  pub fn with_options(mut self, options: T::Option) -> Self {
    self.options = options;
    self
  }

  /// Get the parse result
  pub fn parse(&self) -> ParseResult<T::Result> {
    let parser = T::Parser::new(self.allocator, self.source_text, &self.options);

    parser.parse()
  }
}
