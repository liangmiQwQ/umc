use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

mod kind;
mod lexe;
mod token;

pub struct Html5Lexer<'a> {
  pub(super) allocator: &'a Allocator,
  pub source_text: &'a str,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> Html5Lexer<'a> {
  pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Html5Lexer<'a> {
    Html5Lexer {
      allocator,
      source_text,
      errors: Vec::new(),
    }
  }
}
