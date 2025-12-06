use std::str::Chars;

/// Tracks the current position in source text during parsing.
///
/// This struct maintains a pointer into the source text and provides methods
/// for advancing through the text and accessing remaining characters.
pub struct Source<'a> {
  /// Current byte position in the source text (0-indexed)
  pub pointer: u32,
  /// The complete source text being parsed
  pub source_text: &'a str,
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
  /// assert_eq!(source.pointer, 0);
  /// ```
  pub fn new(source_text: &'a str) -> Source<'a> {
    Source {
      pointer: 0,
      source_text,
    }
  }
}

impl<'a> Source<'a> {
  /// Get an iterator over the remaining characters from the current position.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("abc");
  /// source.advance_bytes(1);
  /// let chars: String = source.get_chars().collect();
  /// assert_eq!(chars, "bc");
  /// ```
  pub fn get_chars(&self) -> Chars<'a> {
    self.source_text[(self.pointer as usize)..].chars()
  }

  /// Advance the pointer forward by the specified number of bytes.
  ///
  /// # Safety
  ///
  /// This method will panic if the byte count doesn't align with UTF-8 character
  /// boundaries or if it advances beyond the end of the source text.
  ///
  /// # Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("hello");
  /// source.advance_bytes(2);
  /// assert_eq!(source.pointer, 2);
  /// ```
  pub fn advance_bytes(&mut self, bytes: u32) {
    let target = self.pointer + bytes;
    self.pointer = target;
  }
}
