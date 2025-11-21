use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;

mod kind;
mod lexe;
mod token;

pub struct Html5Lexer<'a> {
  allocator: &'a Allocator,
  pub errors: Vec<OxcDiagnostic>,
}

impl<'a> Html5Lexer<'a> {
  pub fn new(allocator: &'a Allocator) -> Html5Lexer<'a> {
    Html5Lexer {
      allocator,
      errors: Vec::new(),
    }
  }
}
