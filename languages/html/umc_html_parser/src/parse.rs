use std::iter::Peekable;

use oxc_allocator::{Allocator, Box, Vec as ArenaVec};
use oxc_diagnostics::{LabeledSpan, OxcDiagnostic};
use oxc_parser::Parser as JsParser;
use oxc_span::SourceType;
use umc_html_ast::{
  Attribute, AttributeKey, AttributeValue, Comment, Doctype, Element, Node, Program, Script, Text,
};
use umc_parser::{LanguageParser, ParseResult, ParserImpl, token::Token};
use umc_span::Span;

use crate::{
  Html,
  lexer::{HtmlLexer, HtmlLexerOption, kind::HtmlKind},
  option::HtmlParserOption,
};

/// HTML parser implementation.
///
/// Converts tokens from the lexer into an AST (Abstract Syntax Tree).
/// Uses oxc_allocator for high-performance memory allocation:
/// - All AST nodes are allocated in a bump-based memory arena
/// - String data references the source text directly (zero-copy)
/// - Collections use arena-allocated vectors for cache-friendly traversal
pub struct HtmlParserImpl<'a> {
  /// Arena allocator for AST node allocation.
  /// All Vec and Box types in the AST are allocated from this arena,
  /// providing O(1) allocation and bulk deallocation.
  allocator: &'a Allocator,
  source_text: &'a str,
  options: &'a HtmlParserOption,
  errors: Vec<OxcDiagnostic>,
}

impl<'a> ParserImpl<'a, Html> for HtmlParserImpl<'a> {
  fn new(
    allocator: &'a Allocator,
    source_text: &'a str,
    options: &'a <Html as LanguageParser>::Option,
  ) -> Self {
    HtmlParserImpl {
      allocator,
      source_text,
      options,
      errors: Vec::new(),
    }
  }

  fn parse(mut self) -> ParseResult<Program<'a>> {
    let mut lexer = HtmlLexer::new(
      self.source_text,
      HtmlLexerOption {
        is_embedded_language_tag: &self.options.is_embedded_language_tag,
      },
    );

    // Transfer lexer errors
    self.errors.append(&mut lexer.errors);

    let iter = lexer.tokens().peekable();

    // Parse tokens into AST
    let nodes = self.parse_tokens(iter);

    let Self { errors, .. } = self;

    ParseResult {
      program: nodes,
      errors,
    }
  }
}

/// Represents an element being built during parsing.
/// Uses arena-allocated vectors for children and attributes.
struct ElementBuilder<'a> {
  tag_name: &'a str,
  attributes: ArenaVec<'a, Attribute<'a>>,
  children: ArenaVec<'a, Node<'a>>,
  start: u32,
}

impl<'a> HtmlParserImpl<'a> {
  fn parse_tokens(
    &mut self,
    mut iter: Peekable<impl Iterator<Item = Token<HtmlKind>>>,
  ) -> Program<'a> {
    // Create arena-allocated vector for root nodes
    // Uses bump allocation: O(1) push operations, cache-friendly traversal
    let mut nodes: ArenaVec<'a, Node<'a>> = ArenaVec::new_in(self.allocator);
    let mut element_stack: Vec<ElementBuilder<'a>> = Vec::new();

    while let Some(token) = iter.next() {
      match token.kind {
        HtmlKind::Eof => break,

        HtmlKind::Doctype => {
          let doctype = self.parse_doctype(&token, &mut iter);
          let doctype = Box::new_in(doctype, self.allocator);
          Self::push_node(&mut nodes, &mut element_stack, Node::Doctype(doctype));
        }

        HtmlKind::TagStart => {
          self.parse_opening_tag(&token, &mut iter, &mut nodes, &mut element_stack);
        }

        HtmlKind::CloseTagStart => {
          self.parse_closing_tag(&token, &mut iter, &mut nodes, &mut element_stack);
        }

        HtmlKind::TextContent => {
          let text = self.parse_text(&token);
          let text = Box::new_in(text, self.allocator);
          Self::push_node(&mut nodes, &mut element_stack, Node::Text(text));
        }

        HtmlKind::Comment => {
          let comment = self.parse_comment(&token);
          let comment = Box::new_in(comment, self.allocator);
          Self::push_node(&mut nodes, &mut element_stack, Node::Comment(comment));
        }

        // Other token kinds are handled by the specific parsing functions above

        // Ignore other tokens at content level (whitespace, etc.)
        _ => (),
      }
    }

    // Close any unclosed elements
    while let Some(builder) = element_stack.pop() {
      let end = builder
        .children
        .last()
        .map_or(builder.start, |n| Self::node_end(n));

      self.errors.push(
        OxcDiagnostic::error(format!("Unclosed element: <{}>", builder.tag_name))
          .with_label(Span::new(builder.start, end)),
      );

      let element = Element {
        span: Span::new(builder.start, end),
        tag_name: builder.tag_name,
        attributes: builder.attributes,
        children: builder.children,
      };

      // Push to parent or root
      self.create_and_push_element(element, &mut nodes, &mut element_stack);
    }

    nodes
  }

  /// Parse DOCTYPE declaration with its attributes.
  fn parse_doctype(
    &self,
    doctype_token: &Token<HtmlKind>,
    iter: &mut Peekable<impl Iterator<Item = Token<HtmlKind>>>,
  ) -> Doctype<'a> {
    let start = doctype_token.start;
    let mut end = doctype_token.end;
    // Create arena-allocated vector for DOCTYPE attributes
    let mut attributes: ArenaVec<'a, Attribute<'a>> = ArenaVec::new_in(self.allocator);

    // Parse DOCTYPE attributes until TagEnd
    while let Some(token) = iter.peek() {
      match token.kind {
        HtmlKind::TagEnd => {
          end = token.end;
          iter.next();
          break;
        }
        HtmlKind::Attribute => {
          let attr_token = iter.next().unwrap();
          let attr_text = self.get_token_text(&attr_token);
          // Eq is not expected, all attributes are without value
          attributes.push(Attribute {
            key: AttributeKey {
              span: attr_token.span(),
              value: attr_text,
            },
            value: None,
            span: attr_token.span(),
          });
          end = attr_token.end;
        }
        HtmlKind::Eof => break,
        _ => {
          iter.next();
        }
      }
    }

    Doctype {
      span: Span::new(start, end),
      attributes,
    }
  }

  /// Parse opening tag and push element to stack.
  #[allow(clippy::too_many_lines)]
  fn parse_opening_tag(
    &mut self,
    tag_start_token: &Token<HtmlKind>,
    iter: &mut Peekable<impl Iterator<Item = Token<HtmlKind>>>,
    nodes: &mut ArenaVec<'a, Node<'a>>,
    element_stack: &mut Vec<ElementBuilder<'a>>,
  ) {
    let start = tag_start_token.start;
    let mut tag_name: &'a str = "";
    // Create arena-allocated vector for element attributes
    let mut attributes: ArenaVec<'a, Attribute<'a>> = ArenaVec::new_in(self.allocator);
    let mut is_self_closing = false;

    // Parse element name
    if let Some(token) = iter.peek()
      && token.kind == HtmlKind::ElementName
    {
      let name_token = iter.next().unwrap();
      // Zero-copy: reference source text directly
      tag_name = self.get_token_text(&name_token);
    }

    // Parse attributes until TagEnd or SelfCloseTagEnd
    let mut current_attr_key: Option<AttributeKey<'a>> = None;

    while let Some(token) = iter.peek() {
      match token.kind {
        HtmlKind::TagEnd => {
          iter.next();
          break;
        }
        HtmlKind::SelfCloseTagEnd => {
          is_self_closing = true;
          iter.next();
          break;
        }
        HtmlKind::Attribute => {
          let attr_token = iter.next().unwrap();
          let attr_text = self.get_token_text(&attr_token);

          // If we have a pending attribute key without value, stop storing it because a new attribute is coming
          if let Some(key) = current_attr_key.take() {
            let span = key.span;
            attributes.push(Attribute {
              span,
              key,
              value: None,
            });
          }

          current_attr_key = Some(AttributeKey {
            span: attr_token.span(),
            value: attr_text,
          });
        }
        HtmlKind::Eq => {
          let eq_token = iter.next().unwrap();

          // skip possible whitespace
          if let Some(token) = iter.peek()
            && token.kind == HtmlKind::Whitespace
          {
            iter.next();
          }

          // Next token should be attribute value
          if let Some(value_token) = iter.peek()
            && value_token.kind == HtmlKind::Attribute
          {
            let value_token = iter.next().unwrap();
            if let Some(key) = current_attr_key.take() {
              let value = self.unquote_attribute(&value_token);
              let span = Span::new(key.span.start, value.span.end);
              attributes.push(Attribute {
                span,
                key,
                value: Some(value),
              });
            }
          } else {
            // Handle missing value after =
            self.errors.push(
              OxcDiagnostic::error("Expected attribute value after '='")
                .with_label(Span::new(eq_token.start, eq_token.end)),
            );
          }
        }
        HtmlKind::Eof => break,
        _ => {
          iter.next();
        }
      }
    }

    // Add any remaining attribute without value
    if let Some(key) = current_attr_key.take() {
      let span = key.span;
      attributes.push(Attribute {
        span,
        key,
        value: None,
      });
    }

    // Check for void elements (self-closing by nature)
    if is_self_closing || (self.options.is_void_tag)(tag_name) {
      // Self-closing elements don't go on the stack
      let end = iter
        .peek()
        .map_or(self.source_text.len() as u32, |t| t.start);

      // Create arena-allocated empty vector for children
      let children: ArenaVec<'a, Node<'a>> = ArenaVec::new_in(self.allocator);

      let element = Element {
        span: Span::new(start, end),
        tag_name,
        attributes,
        children,
      };

      // Push to parent or root
      self.create_and_push_element(element, nodes, element_stack);
    } else {
      // Create arena-allocated vector for children
      let children: ArenaVec<'a, Node<'a>> = ArenaVec::new_in(self.allocator);

      // Push to element stack for later matching with closing tag
      element_stack.push(ElementBuilder {
        tag_name,
        attributes,
        children,
        start,
      });
    }
  }

  /// Parse closing tag and pop matching element from stack.
  #[allow(clippy::too_many_lines)]
  fn parse_closing_tag(
    &mut self,
    close_tag_token: &Token<HtmlKind>,
    iter: &mut Peekable<impl Iterator<Item = Token<HtmlKind>>>,
    nodes: &mut ArenaVec<'a, Node<'a>>,
    element_stack: &mut Vec<ElementBuilder<'a>>,
  ) {
    let mut tag_name: &str = "";
    let mut end = if let Some(token) = iter.peek()
      && token.kind == HtmlKind::ElementName
    {
      let name_token = iter.next().unwrap();
      tag_name = self.get_token_text(&name_token);
      name_token.end
    } else {
      close_tag_token.end
    };

    // Skip until TagEnd
    while let Some(token) = iter.peek() {
      match token.kind {
        HtmlKind::TagEnd => {
          end = token.end;
          iter.next();
          break;
        }
        HtmlKind::Eof => break,
        _ => {
          iter.next();
        }
      }
    }

    // Find matching opening tag in stack
    let mut found_index = None;
    for (i, builder) in element_stack.iter().enumerate().rev() {
      if builder.tag_name.eq_ignore_ascii_case(tag_name) {
        found_index = Some(i);
        break;
      }
    }

    if let Some(index) = found_index {
      // Close all elements from top of stack down to the matching one
      while element_stack.len() > index {
        let builder = element_stack.pop().unwrap();
        let elem_end = if element_stack.len() == index {
          end
        } else {
          builder
            .children
            .last()
            .map_or(builder.start, |n| Self::node_end(n))
        };

        let span = Span::new(builder.start, elem_end);

        // Check if this is a script element that should be parsed
        let is_script = builder.tag_name.eq_ignore_ascii_case("script");
        let mut should_parse = is_script && self.options.parse_script.is_some();

        if should_parse {
          for attr in &builder.attributes {
            let key = attr.key.value;
            if key.eq_ignore_ascii_case("src") {
              should_parse = false;
              break;
            }
            #[allow(clippy::collapsible_if)]
            if key.eq_ignore_ascii_case("type") {
              if let Some(val) = &attr.value {
                let v = val.value.to_ascii_lowercase();
                match v.as_str() {
                  ""
                  | "text/javascript"
                  | "application/javascript"
                  | "module"
                  | "text/ecmascript"
                  | "application/ecmascript" => {}
                  _ => {
                    should_parse = false;
                    break;
                  }
                }
              }
            }
          }
        }

        if element_stack.len() > index {
          // This is an implicitly closed element
          self.errors.push(
            OxcDiagnostic::error(format!("Implicitly closed element: <{}>", builder.tag_name))
              .with_label(span),
          );
        }

        if should_parse {
          // Create a Script node with parsed JavaScript
          self.create_and_push_script(
            span,
            builder.tag_name,
            builder.attributes,
            &builder.children,
            nodes,
            element_stack,
          );
        } else {
          // Create a regular Element node
          let element = Element {
            span,
            tag_name: builder.tag_name,
            attributes: builder.attributes,
            children: builder.children,
          };

          // Push to parent or root
          self.create_and_push_element(element, nodes, element_stack);
        }
      }
    } else {
      // No matching opening tag - this is an orphan closing tag
      self.errors.push(
        OxcDiagnostic::error(format!("Unexpected closing tag: </{tag_name}>"))
          .with_label(Span::new(close_tag_token.start, end)),
      );
    }
  }

  /// Parse text content.
  fn parse_text(&self, token: &Token<HtmlKind>) -> Text<'a> {
    Text {
      span: token.span(),
      value: self.get_token_text(token),
    }
  }

  /// Parse comment.
  fn parse_comment(&self, token: &Token<HtmlKind>) -> Comment<'a> {
    let text = self.get_token_text(token);

    // Determine if it's a regular comment or bogus
    let (value, bogus) = if text.starts_with("<!--") {
      // Regular comment: <!-- ... -->
      let content = text
        .strip_prefix("<!--")
        .and_then(|s| s.strip_suffix("-->"))
        .unwrap_or_else(|| text.strip_prefix("<!--").unwrap());
      (content, false)
    } else if text.starts_with("<!") {
      // Bogus comment: <! ... >
      let content = text
        .strip_prefix("<!")
        .and_then(|s| s.strip_suffix(">"))
        .unwrap_or_else(|| text.strip_prefix("<!").unwrap());
      (content, true)
    } else {
      (text, false)
    };

    Comment {
      span: token.span(),
      bogus,
      value,
    }
  }
}

// Some common function and utils
impl<'a> HtmlParserImpl<'a> {
  /// Push a node to the appropriate location (parent element or root).
  fn push_node(
    nodes: &mut ArenaVec<'a, Node<'a>>,
    element_stack: &mut [ElementBuilder<'a>],
    node: Node<'a>,
  ) {
    if let Some(parent) = element_stack.last_mut() {
      parent.children.push(node);
    } else {
      nodes.push(node);
    }
  }

  /// Get the text content of a token.
  fn get_token_text(&self, token: &Token<HtmlKind>) -> &'a str {
    // SAFETY: The source_text has lifetime 'a, and we return a slice of it.
    // This slice is valid as long as the allocator and source text are alive.
    &self.source_text[token.start as usize..token.end as usize]
  }

  /// Remove quotes from attribute value.
  fn unquote_attribute(&self, value: &Token<HtmlKind>) -> AttributeValue<'a> {
    let span = value.span();
    let value = self.get_token_text(value);

    if (value.starts_with('"') && value.ends_with('"'))
      || (value.starts_with('\'') && value.ends_with('\''))
    {
      AttributeValue {
        value: &value[1..value.len() - 1],
        raw: value,
        span,
      }
    } else {
      AttributeValue {
        value,
        raw: value,
        span,
      }
    }
  }

  /// Get the end position of a node.
  fn node_end(node: &Node) -> u32 {
    match node {
      Node::Doctype(d) => d.span.end,
      Node::Element(e) => e.span.end,
      Node::Text(t) => t.span.end,
      Node::Comment(c) => c.span.end,
      Node::Script(s) => s.span.end,
    }
  }

  fn create_and_push_element(
    &self,
    element: Element<'a>,
    nodes: &mut ArenaVec<'a, Node<'a>>,
    element_stack: &mut [ElementBuilder<'a>],
  ) {
    let element = Box::new_in(element, self.allocator);

    if let Some(parent) = element_stack.last_mut() {
      parent.children.push(Node::Element(element));
    } else {
      nodes.push(Node::Element(element));
    }
  }

  /// Create a Script node with parsed JavaScript content.
  ///
  /// Extracts the text content from children (if any), parses it with oxc_parser,
  /// and creates a Script node containing the parsed JavaScript AST.
  fn create_and_push_script(
    &mut self,
    span: Span,
    tag_name: &'a str,
    attributes: ArenaVec<'a, Attribute<'a>>,
    children: &ArenaVec<'a, Node<'a>>,
    nodes: &mut ArenaVec<'a, Node<'a>>,
    element_stack: &mut [ElementBuilder<'a>],
  ) {
    // Extract script content from children
    // If there is a single text node, use it directly (zero-copy)
    // Otherwise, concatenate text nodes and allocate in arena
    let script_content: &str = if children.len() == 1 {
      if let Some(Node::Text(text)) = children.first() {
        text.value
      } else {
        ""
      }
    } else {
      let content = children
        .iter()
        .filter_map(|node| {
          if let Node::Text(text) = node {
            Some(text.value)
          } else {
            None
          }
        })
        .collect::<Vec<_>>()
        .concat();
      self.allocator.alloc_str(&content)
    };

    // Parse the JavaScript content
    let source_type = SourceType::default();
    let parse_options = *self.options.parse_script.as_ref().unwrap();

    let ret = JsParser::new(self.allocator, script_content, source_type)
      .with_options(parse_options)
      .parse();

    // Store JavaScript parsing errors in the main parser errors
    // Adjust error spans to be relative to the HTML source
    let start_offset = children
      .iter()
      .find_map(|node| {
        if let Node::Text(text) = node {
          Some(text.span.start)
        } else {
          None
        }
      })
      .unwrap_or(span.start);

    for mut error in ret.errors {
      if let Some(labels) = error.labels.take() {
        let new_labels = labels
          .into_iter()
          .map(|label| {
            let offset = label.offset() + start_offset as usize;
            let len = label.len();
            let msg = label.label().map(ToString::to_string);
            LabeledSpan::new_with_span(msg, (offset, len))
          })
          .collect();
        error.labels = Some(new_labels);
      }
      self.errors.push(error);
    }

    let script = Script {
      span,
      tag_name,
      attributes,
      program: ret.program,
    };

    let script = Box::new_in(script, self.allocator);

    if let Some(parent) = element_stack.last_mut() {
      parent.children.push(Node::Script(script));
    } else {
      nodes.push(Node::Script(script));
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use insta::assert_snapshot;

  fn parse(source_text: &str) -> String {
    let allocator = Allocator::default();
    let options = HtmlParserOption::default();
    let parser = HtmlParserImpl::new(&allocator, source_text, &options);
    let result = parser.parse();

    format!("Nodes: {:#?}\nErrors: {:#?}", result.program, result.errors)
  }

  #[test]
  fn basic_html() {
    const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Document</title>
</head>
<body>
  <p>Hello World</p>
</body>
</html>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn nested_elements() {
    const HTML: &str = r#"<div>
  <p>Paragraph 1</p>
  <p>Paragraph 2</p>
</div>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn self_closing_elements() {
    const HTML: &str = r#"<div>
  <br>
  <img src="test.jpg" alt="Test">
  <input type="text" />
</div>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn comments() {
    const HTML: &str = r#"<!-- This is a comment -->
<div>Content</div>
<!-- Another comment -->
<! This is a bogus comment >
<!Bogus Comment Too>
"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn attribute_with_whitespaces() {
    const HTML: &str = r#"<div class = "test" a= "b">Content</div>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn multiple_no_value_attributes() {
    const HTML: &str = r#"<input checked disabled readonly>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn void_elements() {
    const HTML: &str = r#"<div>
  <br>
  <img src="test.jpg" alt="Test">
  <input type="text">
  <meta>Text</meta> <!-- Should Error -->
</div>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn self_close_tags() {
    const HTML: &str = r#"<div/><p />"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn no_closing_tag() {
    const HTML: &str = r#"<div>
  <p>Unclosed paragraph
</div>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn orphan_closing_tag() {
    const HTML: &str = r#"<div>Content</div>
</span>
<p>More content</p>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn incomplete_attribute() {
    const HTML: &str = r#"<div class=></div>"#;
    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn script_parsing() {
    const HTML: &str = r#"<script>
      const a = 1;
      function b() { return a + 1; }
    </script>"#;
    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn script_parsing_error() {
    const HTML: &str = r#"<script>
      const a =;
    </script>"#;
    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn script_with_src() {
    const HTML: &str = r#"<script src="foo.js"></script>"#;
    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn script_with_invalid_type() {
    const HTML: &str = r#"<script type="foo/bar">console.log(1)</script>"#;
    assert_snapshot!(parse(HTML));
  }
}
