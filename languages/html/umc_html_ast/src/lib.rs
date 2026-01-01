//! HTML Abstract Syntax Tree (AST) node definitions.
//!
//! This crate defines the AST node types used to represent parsed HTML documents.
//! It includes nodes for elements, text, comments, DOCTYPE declarations, and attributes.
//!
//! # Arena Allocation
//!
//! All AST types use arena allocation via [`oxc_allocator`] for high performance:
//! - String data uses `&'a str` references to the source text (zero-copy)
//! - Collections use `oxc_allocator::Vec<'a, T>` for cache-friendly traversal
//! - Memory is released in bulk when the allocator is dropped (no individual Drop calls)
//!
//! # Example
//!
//! ```
//! use oxc_allocator::Allocator;
//! use umc_html_ast::{Element, Node, Text};
//! use umc_span::Span;
//!
//! let allocator = Allocator::default();
//!
//! let text_node = Text {
//!     span: Span::new(0, 5),
//!     value: "Hello",
//! };
//!
//! let element = Element {
//!     span: Span::new(0, 20),
//!     tag_name: "div",
//!     attributes: oxc_allocator::Vec::new_in(&allocator),
//!     children: oxc_allocator::Vec::new_in(&allocator),
//! };
//! ```

use oxc_allocator::{Box, Vec};
use umc_span::Span;

/// HTML AST node types.
///
/// Represents the different kinds of nodes that can appear in an HTML document.
/// Each variant wraps a specific node type with its associated data.
///
/// The lifetime `'a` is tied to the allocator that owns the memory for this AST.
#[derive(Debug)]
pub enum Node<'a> {
  /// HTML DOCTYPE declaration
  Doctype(Box<'a, Doctype<'a>>),
  /// HTML element with tag, attributes, and children
  Element(Box<'a, Element<'a>>),
  /// Text content node
  Text(Box<'a, Text<'a>>),
  /// HTML comment node
  Comment(Box<'a, Comment<'a>>),
  /// Script element with parsed JavaScript content
  Script(Box<'a, Script<'a>>),
}

/// An alias for a vector of HTML AST nodes.
///
/// This type is used to represent the root of an HTML document.
pub type Program<'a> = Vec<'a, Node<'a>>;

/// HTML DOCTYPE declaration node.
///
/// Represents the `<!DOCTYPE ...>` declaration at the beginning of HTML documents.
/// For example: `<!DOCTYPE html>`
///
/// The lifetime `'a` is tied to the allocator that owns the memory.
#[derive(Debug)]
pub struct Doctype<'a> {
  /// Source location of this DOCTYPE declaration
  pub span: Span,
  /// Attributes of the DOCTYPE (rarely used in modern HTML5).
  /// Stored in arena-allocated vector for cache-friendly traversal.
  pub attributes: Vec<'a, Attribute<'a>>,
}

/// HTML element node.
///
/// Represents an HTML element with its tag name, attributes, and child nodes.
/// For example: `<div class="container"><p>Hello</p></div>`
///
/// The lifetime `'a` is tied to the allocator that owns the memory.
#[derive(Debug)]
pub struct Element<'a> {
  /// Source location of this element
  pub span: Span,
  /// Tag name (e.g., "div", "span", "html").
  /// References the original source text (zero-copy).
  pub tag_name: &'a str,
  /// Element attributes (e.g., class, id, href).
  /// Stored in arena-allocated vector for cache-friendly traversal.
  pub attributes: Vec<'a, Attribute<'a>>,
  /// Child nodes contained within this element.
  /// Stored in arena-allocated vector for cache-friendly traversal.
  pub children: Vec<'a, Node<'a>>,
}

/// Text content node.
///
/// Represents plain text content within HTML elements.
/// For example, the "Hello World" in `<span>Hello World</span>`
///
/// The lifetime `'a` is tied to the allocator that owns the memory.
#[derive(Debug)]
pub struct Text<'a> {
  /// Source location of this text node
  pub span: Span,
  /// The text content. References the original source text (zero-copy).
  pub value: &'a str,
}

/// HTML comment node.
///
/// Represents an HTML comment. For example: `<!-- This is a comment -->`
///
/// The lifetime `'a` is tied to the allocator that owns the memory.
#[derive(Debug)]
pub struct Comment<'a> {
  /// Source location of this comment
  pub span: Span,
  /// Whether this comment is bogus e.g. <! hello world > (https://html.spec.whatwg.org/multipage/parsing.html#bogus-comment-state)
  pub bogus: bool,
  /// The comment text content (without the `<!--` and `-->` delimiters).
  /// References the original source text (zero-copy).
  pub value: &'a str,
}

/// Script element with parsed JavaScript content.
///
/// Represents a `<script>` element where the JavaScript content has been
/// parsed by `oxc_parser` into an AST.
///
/// The lifetime `'a` is tied to the allocator that owns the memory.
#[derive(Debug)]
pub struct Script<'a> {
  /// Source location of this script element
  pub span: Span,
  /// Tag name (always "script", case-insensitive in source)
  pub tag_name: &'a str,
  /// Element attributes (e.g., type, src, defer)
  pub attributes: Vec<'a, Attribute<'a>>,
  /// The parsed JavaScript program from oxc_parser
  pub program: oxc_ast::ast::Program<'a>,
}

/// HTML element attribute.
///
/// Represents a key-value pair attribute on an HTML element.
/// For example: `class="container"` or `href="https://example.com"`
///
/// The value will be empty if no attribute value got after `=`
/// like `<div class>` will get ```Attribute { key: "class", value: "" }```
///
/// The lifetime `'a` is tied to the allocator that owns the memory.
#[derive(Debug)]
pub struct Attribute<'a> {
  /// Source location of this attribute
  pub span: Span,
  /// Attribute name (e.g., "class", "id", "href").
  /// References the original source text.
  pub key: AttributeKey<'a>,
  /// Attribute value. References the original source text.
  pub value: Option<AttributeValue<'a>>,
}

#[derive(Debug)]
pub struct AttributeKey<'a> {
  pub span: Span,
  pub value: &'a str,
}

#[derive(Debug)]
pub struct AttributeValue<'a> {
  pub span: Span,
  pub value: &'a str,
  pub raw: &'a str,
}
