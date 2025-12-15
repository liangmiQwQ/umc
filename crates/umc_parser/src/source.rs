/// Tracks the current position in source text during parsing.
///
/// This struct maintains a pointer into the source text and provides methods
/// for advancing through the text and accessing remaining characters.
pub struct Source<'a> {
  /// Current byte position in the source text (0-indexed)
  pub pointer: u32,
  /// The complete source text being parsed
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
  pub fn get(&self, index: u32) -> Option<u8> {
    self.source_text.get(index as usize).copied()
  }

  pub fn starts_with(&self, bytes: &[u8]) -> bool {
    self.source_text[self.pointer as usize..].starts_with(bytes)
  }

  pub fn starts_with_lowercase(&self, bytes: &[u8]) -> bool {
    self.source_text[self.pointer as usize..]
      .to_ascii_lowercase()
      .starts_with(bytes)
  }

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
  /// source.to(2);
  /// assert_eq!(source.pointer, 2);
  /// ```
  pub fn to(&mut self, index: u32) {
    self.pointer = index;
  }

  pub fn advance(&mut self, diff: u32) {
    self.pointer += diff;
  }
}
