/// Tracks the current position in source text during parsing.
///
/// This struct maintains a pointer into the source text and provides methods
/// for advancing through the text and accessing remaining characters.
pub struct Source<'a> {
  /// Current byte position in the source text (0-indexed)
  pub pointer: u32,
  /// The complete source text being parsed
  /// Use slice instead of str since we always do byte-level operations
  pub source_text: &'a [u8],
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
      source_text: source_text.as_bytes(),
    }
  }
}

impl<'a> Source<'a> {
  /// Get the byte at the given index
  ///
  /// ## Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let source = Source::new("hello");
  /// assert_eq!(source.get(0), Some(b'h'));
  /// assert_eq!(source.get(5), None);
  /// ```
  #[inline]
  pub fn get(&self, index: u32) -> Option<u8> {
    self.source_text.get(index as usize).copied()
  }

  /// Check if the remaining source text starts with the given bytes
  ///
  /// ## Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let source = Source::new("hello");
  /// assert!(source.starts_with(b"he"));
  /// assert!(!source.starts_with(b"hl"));
  /// ```
  #[inline]
  pub fn starts_with(&self, bytes: &[u8]) -> bool {
    self.source_text[self.pointer as usize..].starts_with(bytes)
  }

  /// Check if the remaining source text starts with the given bytes
  /// Ignore case, but you need to pass in the bytes in lowercase
  ///
  /// ## Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let source = Source::new("HELLO");
  /// assert!(source.starts_with_lowercase(b"he"));
  /// assert!(!source.starts_with_lowercase(b"hl"));
  /// ```
  #[inline]
  pub fn starts_with_lowercase(&self, bytes: &[u8]) -> bool {
    self.source_text[self.pointer as usize..]
      .to_ascii_lowercase()
      .starts_with(bytes)
  }

  /// Get the remaining source text which is after the current pointer location
  ///
  /// ## Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("hello");
  /// source.advance(1);
  /// assert_eq!(source.rest(), b"ello");
  /// ```
  #[inline]
  pub fn rest(&self) -> &[u8] {
    &self.source_text[self.pointer as usize..]
  }

  /// Set the pointer location
  ///
  /// ## Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("hello");
  /// source.advance(1);
  /// source.to(2);
  /// assert_eq!(source.pointer, 2);
  /// ```
  #[inline]
  pub fn to(&mut self, index: u32) {
    self.pointer = index;
  }

  /// Advance the pointer by a given amount
  /// Based on current pointer location
  ///
  /// ## Example
  ///
  /// ```
  /// use umc_parser::source::Source;
  ///
  /// let mut source = Source::new("hello");
  /// source.advance(2);
  /// source.advance(2);
  /// assert_eq!(source.pointer, 4);
  /// ```
  #[inline]
  pub fn advance(&mut self, diff: u32) {
    self.pointer += diff;
  }
}
