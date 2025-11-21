use std::fmt::{self, Display};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Html5Kind {
  #[default]
  Eof = 0,

  // Tags
  TagStart,      // <
  TagEnd,        // >
  CloseTagStart, // </
  SelfCloseEnd,  // />

  // Identifier
  ElementName,    // div, span, html, etc.
  AttributeName,  // class, id, src, etc.
  AttributeValue, // like the "w-full" of class="w-full"

  // Texts
  TextContent, // like the "Hello World" of <span>Hello World</span>
  Comment,     // <!-- ... -->

  // Misc
  Eq,   // =
  Skip, // Whitespace, line breaks

  // Special
  Doctype, // <!DOCTYPE ...>
}

use Html5Kind::*;

impl Html5Kind {
  #[inline]
  #[must_use]
  pub fn is_eof(self) -> bool {
    self == Eof
  }

  #[must_use]
  pub fn to_str(self) -> &'static str {
    match self {
      Eof => "EOF",

      TagStart => "<",
      TagEnd => ">",
      CloseTagStart => "</",
      SelfCloseEnd => "/>",

      ElementName => "element-name",
      AttributeName => "attribute-name",
      AttributeValue => "attribute-value",

      TextContent => "text",
      Comment => "<!-- comment -->",

      Eq => "=",
      Skip => "Skipped",

      Doctype => "<!DOCTYPE>",
    }
  }
}

impl Display for Html5Kind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.to_str().fmt(f)
  }
}
