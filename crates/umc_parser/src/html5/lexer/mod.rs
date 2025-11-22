use crate::html5::lexer::source::Source;
use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

mod kind;
mod lexe;
mod source;
mod token;

pub(crate) struct Html5Lexer<'a> {
  allocator: &'a Allocator,
  source: Source<'a>,
  errors: Vec<OxcDiagnostic>,
}

impl<'a> Html5Lexer<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Html5Lexer<'a> {
    Html5Lexer {
      allocator,
      source: Source::new(source_text),
      errors: Vec::new(),
    }
  }
}
