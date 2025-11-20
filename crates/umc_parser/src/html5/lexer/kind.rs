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
  TagName,        // div, span, html, etc.
  AttributeName,  // class, id, src, etc.
  AttributeValue, // like the "w-full" of class="w-full"

  // Texts
  Text,    // like the "Hello World" of <span>Hello World</span>
  Comment, // <!-- ... -->

  // Special
  Doctype, // <!DOCTYPE ...>
}

use Html5Kind::*;

impl Html5Kind {
  #[inline]
  pub fn is_eof(self) -> bool {
    self == Eof
  }

  pub fn to_str(self) -> &'static str {
    match self {
      Eof => "EOF",

      TagStart => "<",
      TagEnd => ">",
      CloseTagStart => "</",
      SelfCloseEnd => "/>",

      TagName => "tagname",
      AttributeName => "attribute-name",
      AttributeValue => "attribute-value",

      Text => "text",
      Comment => "<!-- Comment -->",

      // Special
      Doctype => "<!DOCTYPE doctype>",
    }
  }
}

impl Display for Html5Kind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.to_str().fmt(f)
  }
}
