use std::fmt::{self, Display};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum HtmlKind {
  #[default]
  Eof = 0,

  // Tags
  TagStart,        // <
  TagEnd,          // >
  CloseTagStart,   // </
  SelfCloseTagEnd, // />
  Doctype,         // <!DOCTYPE

  // Identifier
  ElementName, // div, span, html, etc.
  Attribute,   // both for attribute value and attribute name

  // Texts
  TextContent, // like the "Hello World" of <span>Hello World</span>
  Comment,     // <!-- ... -->

  // Misc
  Eq,         // =
  Whitespace, // Whitespace, line breaks
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
