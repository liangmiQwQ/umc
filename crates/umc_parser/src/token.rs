use umc_span::Span;

#[derive(Debug, PartialEq)]
pub struct Token<T> {
  pub kind: T,
  pub start: u32,
  pub end: u32,
}

impl<T> Token<T> {
  pub fn span(&self) -> Span {
    Span::new(self.start, self.end)
  }
}
