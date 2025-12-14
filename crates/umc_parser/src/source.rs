use std::iter::Peekable;
use std::str::Chars;

/// Tracks the current position in source text during parsing.
///
/// This struct maintains a peekable iterator over the source text and provides
/// methods for peeking and consuming characters while tracking byte position.
pub struct Source<'a> {
  /// Current byte position in the source text (0-indexed)
  pub position: u32,
  /// The complete source text being parsed
  pub source_text: &'a str,
  /// Peekable iterator over remaining characters
  chars: Peekable<Chars<'a>>,
}

impl<'a> Source<'a> {
  /// Create a new source tracker starting at position 0.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let source = Source::new("<html></html>");
  /// assert_eq!(source.position, 0);
  /// ```
  pub fn new(source_text: &'a str) -> Source<'a> {
    Source {
      position: 0,
      source_text,
      chars: source_text.chars().peekable(),
    }
  }

  /// Peek at the next character without consuming it.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("abc");
  /// assert_eq!(source.peek(), Some(&'a'));
  /// assert_eq!(source.peek(), Some(&'a')); // Still 'a', not consumed
  /// ```
  #[inline]
  pub fn peek(&mut self) -> Option<&char> {
    self.chars.peek()
  }

  /// Consume and return the next character, advancing position.
  /// Named `bump` instead of `next` to avoid confusion with Iterator::next.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("abc");
  /// assert_eq!(source.bump(), Some('a'));
  /// assert_eq!(source.position, 1);
  /// assert_eq!(source.bump(), Some('b'));
  /// assert_eq!(source.position, 2);
  /// ```
  #[inline]
  pub fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;
    self.position += c.len_utf8() as u32;
    Some(c)
  }

  /// Check if the source has reached the end.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("a");
  /// assert!(!source.is_eof());
  /// source.bump();
  /// assert!(source.is_eof());
  /// ```
  #[inline]
  pub fn is_eof(&mut self) -> bool {
    self.chars.peek().is_none()
  }

  /// Get the remaining source text from current position.
  /// Useful for lookahead patterns like starts_with.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("hello");
  /// source.bump(); // consume 'h'
  /// assert_eq!(source.remaining(), "ello");
  /// ```
  #[inline]
  pub fn remaining(&self) -> &'a str {
    &self.source_text[self.position as usize..]
  }

  /// Advance the position by `n` bytes.
  /// This is useful when you want to skip a chunk of text that you have already processed
  /// or identified using string slice methods (like `find` or `memchr`).
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("hello world");
  /// source.advance(6);
  /// assert_eq!(source.position, 6);
  /// assert_eq!(source.peek(), Some(&'w'));
  /// ```
  #[inline]
  pub fn advance(&mut self, n: u32) {
    self.position += n;
    self.chars = self.source_text[self.position as usize..]
      .chars()
      .peekable();
  }
}
