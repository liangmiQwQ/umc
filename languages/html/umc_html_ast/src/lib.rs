//! HTML Abstract Syntax Tree (AST) node definitions.
//!
//! This crate defines the AST node types used to represent parsed HTML documents.
//! It includes nodes for elements, text, comments, DOCTYPE declarations, and attributes.
//!
//! # Example
//!
//! ```
//! use umc_html_ast::{Element, Node, Text};
//! use umc_span::Span;
//!
//! let text_node = Text {
//!     span: Span::new(0, 5),
//!     value: "Hello".to_string(),
//! };
//!
//! let element = Element {
//!     span: Span::new(0, 20),
//!     tag_name: "div".to_string(),
//!     attributes: vec![],
//!     children: vec![Node::Text(text_node)],
//! };
//! ```

use umc_span::Span;

/// HTML AST node types.
///
/// Represents the different kinds of nodes that can appear in an HTML document.
/// Each variant wraps a specific node type with its associated data.
pub enum Node {
  /// HTML DOCTYPE declaration
  Doctype(Doctype),
  /// HTML element with tag, attributes, and children
  Element(Element),
  /// Text content node
  Text(Text),
  /// HTML comment node
  Comment(Comment),
}

/// HTML DOCTYPE declaration node.
///
/// Represents the `<!DOCTYPE ...>` declaration at the beginning of HTML documents.
/// For example: `<!DOCTYPE html>`
pub struct Doctype {
  /// Source location of this DOCTYPE declaration
  pub span: Span,
  /// Attributes of the DOCTYPE (rarely used in modern HTML5)
  pub attributes: Vec<Attribute>,
}

/// HTML element node.
///
/// Represents an HTML element with its tag name, attributes, and child nodes.
/// For example: `<div class="container"><p>Hello</p></div>`
pub struct Element {
  /// Source location of this element
  pub span: Span,
  /// Tag name (e.g., "div", "span", "html")
  pub tag_name: String,
  /// Element attributes (e.g., class, id, href)
  pub attributes: Vec<Attribute>,
  /// Child nodes contained within this element
  pub children: Vec<Node>,
}

/// Text content node.
///
/// Represents plain text content within HTML elements.
/// For example, the "Hello World" in `<span>Hello World</span>`
pub struct Text {
  /// Source location of this text node
  pub span: Span,
  /// The text content
  pub value: String,
}

/// HTML comment node.
///
/// Represents an HTML comment. For example: `<!-- This is a comment -->`
pub struct Comment {
  /// Source location of this comment
  pub span: Span,
  /// Whether this comment is bogus e.g. <! hello world > (https://html.spec.whatwg.org/multipage/parsing.html#bogus-comment-state)
  pub bogus: bool,
  /// The comment text content (without the `<!--` and `-->` delimiters)
  pub value: String,
}

/// HTML element attribute.
///
/// Represents a key-value pair attribute on an HTML element.
/// For example: `class="container"` or `href="https://example.com"`
///
/// The value will be empty if no attribute value got after `=`
/// like `<div class>` will get ```Attribute { key: "class", value: "" }```
pub struct Attribute {
  /// Attribute name (e.g., "class", "id", "href")
  pub key: String,
  /// Attribute value
  pub value: String,
}
