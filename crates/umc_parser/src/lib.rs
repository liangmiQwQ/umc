use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

pub mod html5;

pub trait Language: Sized {
  type Result;
  type Option: Default;
  type Parser: ParserImpl<Self>;
}

pub trait ParserImpl<T: Language> {
  fn new(allocator: &Allocator, source_text: &str, options: &T::Option) -> Self;
  fn parse(self) -> T::Result;
}

pub struct Parser<'a, T: Language> {
  allocator: &'a Allocator,
  source_text: &'a str,
  options: T::Option,
}

pub struct ParseResult<T> {
  pub program: Program<T>,
  pub errors: Vec<OxcDiagnostic>,
  pub panicked: bool,
}

pub struct Program<T> {
  pub body: Vec<T>,
}

impl<'a, T: Language> Parser<'a, T> {
  /// Creat the umc parser
  ///
  /// # Parameters
  /// - `allocator`: [Memory arena](oxc_allocator::Allocator) for allocating AST nodes
  /// - `source_text`: Source code to parse
  ///
  /// # Examples
  /// ```rust
  /// use oxc_allocator::Allocator;
  /// use umc_parser::Parser;
  /// use umc_parser::html5::Html5;
  ///
  /// let allocator = Allocator::default();
  /// let parser = Parser::<Html5>::new(&allocator, "<html> Hello World </html>");
  /// ```
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

  pub fn parse(&self) -> ParseResult<T::Result> {
    let parser = T::Parser::new(self.allocator, self.source_text, &self.options);

    todo!("{:p}", &parser.parse())
  }
}
