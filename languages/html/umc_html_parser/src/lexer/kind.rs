use std::fmt::{self, Display};

/// HTML token kinds used by the lexer.
///
/// Represents the different types of tokens that can be encountered when
/// lexing HTML source code. Each variant corresponds to a specific syntactic
/// element in HTML.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum HtmlKind {
  /// End of file marker
  #[default]
  Eof = 0,

  // Tags
  /// Opening tag start: `<`
  TagStart,
  /// Tag end: `>`
  TagEnd,
  /// Closing tag start: `</`
  CloseTagStart,
  /// Self-closing tag end: `/>`
  SelfCloseTagEnd,
  /// DOCTYPE declaration: `<!DOCTYPE`
  Doctype,

  // Identifier
  /// HTML element name (e.g., div, span, html)
  ElementName,
  /// Attribute name or value
  Attribute,

  // Texts
  /// Text content within elements
  TextContent,
  /// HTML comment: `<!-- ... -->`
  Comment,

  // Misc
  /// Equals sign in attributes: `=`
  Eq,
  /// Whitespace and line breaks
  Whitespace,
}

use HtmlKind::*;

impl HtmlKind {
  #[must_use]
  pub fn to_str(self) -> &'static str {
    match self {
      Eof => "EOF",

      TagStart => "<",
      TagEnd => ">",
      CloseTagStart => "</",
      SelfCloseTagEnd => "/>",

      ElementName => "element-name",
      Attribute => "attribute",

      TextContent => "text",
      Comment => "<!-- comment -->",

      Eq => "=",
      Whitespace => "Whitespace",

      Doctype => "<!DOCTYPE",
    }
  }
}

impl Display for HtmlKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.to_str().fmt(f)
  }
}
