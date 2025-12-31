use umc_span::Span;

/// A generic token with position information.
///
/// Represents a lexical token with a kind/type and its position in the source text.
/// The position is stored as byte offsets (start and end).
///
/// # Type Parameters
///
/// * `T` - The token kind type (e.g., `HtmlKind` for HTML tokens)
#[derive(Debug, PartialEq, Eq)]
pub struct Token<T> {
  /// The kind or type of this token
  pub kind: T,
  /// Starting byte position in the source text (inclusive)
  pub start: u32,
  /// Ending byte position in the source text (exclusive)
  pub end: u32,
}

impl<T> Token<T> {
  /// Get the span (position range) of this token.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::token::Token;
  ///
  /// #[derive(Debug, PartialEq)]
  /// enum Kind { Text }
  ///
  /// let token = Token { kind: Kind::Text, start: 0, end: 5 };
  /// let span = token.span();
  /// assert_eq!(span.start, 0);
  /// assert_eq!(span.end, 5);
  /// ```
  pub const fn span(&self) -> Span {
    Span::new(self.start, self.end)
  }
}
