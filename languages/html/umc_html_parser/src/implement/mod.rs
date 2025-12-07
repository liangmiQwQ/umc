use std::iter::Peekable;

use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;
use umc_html_ast::{Attribute, Comment, Doctype, Element, Node, Text};
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
/// Uses oxc_allocator for high-performance memory allocation.
pub struct HtmlParserImpl<'a> {
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

  fn parse(mut self) -> ParseResult<Vec<Node>> {
    let mut lexer = HtmlLexer::new(
      self.allocator,
      self.source_text,
      HtmlLexerOption {
        embedded_language_tags: &self.options.embedded_language_tags,
      },
    );

    // Transfer lexer errors
    self.errors.append(&mut lexer.errors);

    let iter = lexer.tokens().peekable();

    // Parse tokens into AST
    let nodes = self.parse_tokens(iter);

    ParseResult {
      program: nodes,
      errors: std::mem::take(&mut self.errors),
    }
  }
}

/// Represents an element being built during parsing.
struct ElementBuilder {
  tag_name: String,
  attributes: Vec<Attribute>,
  children: Vec<Node>,
  start: u32,
}

impl<'a> HtmlParserImpl<'a> {
  fn parse_tokens(&mut self, iter: Peekable<impl Iterator<Item = Token<HtmlKind>>>) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut element_stack: Vec<ElementBuilder> = Vec::new();
    let mut iter = iter;

    while let Some(token) = iter.next() {
      match token.kind {
        HtmlKind::Eof => break,

        HtmlKind::Doctype => {
          let doctype = self.parse_doctype(&token, &mut iter);
          self.push_node(&mut nodes, &mut element_stack, Node::Doctype(doctype));
        }

        HtmlKind::TagStart => {
          self.parse_opening_tag(&token, &mut iter, &mut element_stack);
        }

        HtmlKind::CloseTagStart => {
          self.parse_closing_tag(&token, &mut iter, &mut nodes, &mut element_stack);
        }

        HtmlKind::TextContent => {
          let text = self.parse_text(&token);
          self.push_node(&mut nodes, &mut element_stack, Node::Text(text));
        }

        HtmlKind::Comment => {
          let comment = self.parse_comment(&token);
          self.push_node(&mut nodes, &mut element_stack, Node::Comment(comment));
        }

        // Ignore other tokens at content level (whitespace, etc.)
        _ => {}
      }
    }

    // Close any unclosed elements
    while let Some(builder) = element_stack.pop() {
      let end = builder
        .children
        .last()
        .map(|n| self.node_end(n))
        .unwrap_or(builder.start);

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
      if let Some(parent) = element_stack.last_mut() {
        parent.children.push(Node::Element(element));
      } else {
        nodes.push(Node::Element(element));
      }
    }

    nodes
  }

  /// Parse DOCTYPE declaration with its attributes.
  fn parse_doctype(
    &mut self,
    doctype_token: &Token<HtmlKind>,
    iter: &mut Peekable<impl Iterator<Item = Token<HtmlKind>>>,
  ) -> Doctype {
    let start = doctype_token.start;
    let mut end = doctype_token.end;
    let mut attributes = Vec::new();

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
          attributes.push(Attribute {
            key: attr_text.to_string(),
            value: String::new(),
          });
          end = attr_token.end;
        }
        HtmlKind::Eq => {
          iter.next();
          // Next token should be attribute value
          if let Some(value_token) = iter.peek()
            && value_token.kind == HtmlKind::Attribute
          {
            let value_token = iter.next().unwrap();
            let value_text = self.get_token_text(&value_token);
            // Update last attribute's value
            if let Some(attr) = attributes.last_mut() {
              attr.value = self.unquote_attribute(value_text);
            }
            end = value_token.end;
          }
        }
        HtmlKind::Whitespace => {
          iter.next();
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
  fn parse_opening_tag(
    &mut self,
    tag_start_token: &Token<HtmlKind>,
    iter: &mut Peekable<impl Iterator<Item = Token<HtmlKind>>>,
    element_stack: &mut Vec<ElementBuilder>,
  ) {
    let start = tag_start_token.start;
    let mut tag_name = String::new();
    let mut attributes = Vec::new();
    let mut is_self_closing = false;

    // Parse element name
    if let Some(token) = iter.peek()
      && token.kind == HtmlKind::ElementName
    {
      let name_token = iter.next().unwrap();
      tag_name = self.get_token_text(&name_token).to_string();
    }

    // Parse attributes until TagEnd or SelfCloseTagEnd
    let mut current_attr_key: Option<String> = None;

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

          // If we have a pending attribute key without value, add it
          if let Some(key) = current_attr_key.take() {
            attributes.push(Attribute {
              key,
              value: String::new(),
            });
          }

          current_attr_key = Some(attr_text.to_string());
        }
        HtmlKind::Eq => {
          iter.next();
          // Next token should be attribute value
          if let Some(value_token) = iter.peek()
            && value_token.kind == HtmlKind::Attribute
          {
            let value_token = iter.next().unwrap();
            let value_text = self.get_token_text(&value_token);

            if let Some(key) = current_attr_key.take() {
              attributes.push(Attribute {
                key,
                value: self.unquote_attribute(value_text),
              });
            }
          }
        }
        HtmlKind::Whitespace => {
          iter.next();
        }
        HtmlKind::Eof => break,
        _ => {
          iter.next();
        }
      }
    }

    // Add any remaining attribute without value
    if let Some(key) = current_attr_key.take() {
      attributes.push(Attribute {
        key,
        value: String::new(),
      });
    }

    // Check for void elements (self-closing by nature)
    let is_void = Self::is_void_element(&tag_name);

    if is_self_closing || is_void {
      // Self-closing elements don't go on the stack
      let end = iter
        .peek()
        .map(|t| t.start)
        .unwrap_or(self.source_text.len() as u32);

      let element = Element {
        span: Span::new(start, end),
        tag_name,
        attributes,
        children: Vec::new(),
      };

      // Push to parent or root (will be handled in push_node via element_stack)
      if let Some(parent) = element_stack.last_mut() {
        parent.children.push(Node::Element(element));
      } else {
        // This is a root-level element, but we don't have nodes here
        // We need to push it to the stack and immediately pop it
        element_stack.push(ElementBuilder {
          tag_name: element.tag_name.clone(),
          attributes: Vec::new(),
          children: vec![Node::Element(element)],
          start,
        });
        let builder = element_stack.pop().unwrap();
        // Return the element from the builder's children
        if let Some(parent) = element_stack.last_mut() {
          parent.children.extend(builder.children);
        }
      }
    } else {
      // Push to element stack for later matching with closing tag
      element_stack.push(ElementBuilder {
        tag_name,
        attributes,
        children: Vec::new(),
        start,
      });
    }
  }

  /// Parse closing tag and pop matching element from stack.
  fn parse_closing_tag(
    &mut self,
    close_tag_token: &Token<HtmlKind>,
    iter: &mut Peekable<impl Iterator<Item = Token<HtmlKind>>>,
    nodes: &mut Vec<Node>,
    element_stack: &mut Vec<ElementBuilder>,
  ) {
    let mut tag_name = String::new();
    let mut end = close_tag_token.end;

    // Parse element name
    if let Some(token) = iter.peek()
      && token.kind == HtmlKind::ElementName
    {
      let name_token = iter.next().unwrap();
      tag_name = self.get_token_text(&name_token).to_string();
      end = name_token.end;
    }

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
      if builder.tag_name.eq_ignore_ascii_case(&tag_name) {
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
            .map(|n| self.node_end(n))
            .unwrap_or(builder.start)
        };

        let element = Element {
          span: Span::new(builder.start, elem_end),
          tag_name: builder.tag_name,
          attributes: builder.attributes,
          children: builder.children,
        };

        if element_stack.len() > index {
          // This is an implicitly closed element
          self.errors.push(
            OxcDiagnostic::error(format!("Implicitly closed element: <{}>", element.tag_name))
              .with_label(element.span),
          );
        }

        // Push to parent or root
        if let Some(parent) = element_stack.last_mut() {
          parent.children.push(Node::Element(element));
        } else {
          nodes.push(Node::Element(element));
        }
      }
    } else {
      // No matching opening tag - this is an orphan closing tag
      self.errors.push(
        OxcDiagnostic::error(format!("Unexpected closing tag: </{}>", tag_name))
          .with_label(Span::new(close_tag_token.start, end)),
      );
    }
  }

  /// Parse text content.
  fn parse_text(&self, token: &Token<HtmlKind>) -> Text {
    Text {
      span: token.span(),
      value: self.get_token_text(token).to_string(),
    }
  }

  /// Parse comment.
  fn parse_comment(&self, token: &Token<HtmlKind>) -> Comment {
    let text = self.get_token_text(token);

    // Determine if it's a regular comment or bogus
    let (value, bogus) = if text.starts_with("<!--") {
      // Regular comment: <!-- ... -->
      let content = text
        .strip_prefix("<!--")
        .and_then(|s| s.strip_suffix("-->"))
        .unwrap_or(text.strip_prefix("<!--").unwrap());
      (content.to_string(), false)
    } else if text.starts_with("<!") {
      // Bogus comment: <! ... >
      let content = text
        .strip_prefix("<!")
        .and_then(|s| s.strip_suffix(">"))
        .unwrap_or(text.strip_prefix("<!").unwrap());
      (content.to_string(), true)
    } else {
      (text.to_string(), false)
    };

    Comment {
      span: token.span(),
      bogus,
      value,
    }
  }

  /// Push a node to the appropriate location (parent element or root).
  fn push_node(&self, nodes: &mut Vec<Node>, element_stack: &mut [ElementBuilder], node: Node) {
    if let Some(parent) = element_stack.last_mut() {
      parent.children.push(node);
    } else {
      nodes.push(node);
    }
  }

  /// Get the text content of a token.
  fn get_token_text(&self, token: &Token<HtmlKind>) -> &str {
    &self.source_text[token.start as usize..token.end as usize]
  }

  /// Remove quotes from attribute value.
  fn unquote_attribute(&self, value: &str) -> String {
    if (value.starts_with('"') && value.ends_with('"'))
      || (value.starts_with('\'') && value.ends_with('\''))
    {
      value[1..value.len() - 1].to_string()
    } else {
      value.to_string()
    }
  }

  /// Get the end position of a node.
  fn node_end(&self, node: &Node) -> u32 {
    match node {
      Node::Doctype(d) => d.span.end,
      Node::Element(e) => e.span.end,
      Node::Text(t) => t.span.end,
      Node::Comment(c) => c.span.end,
    }
  }

  /// Check if an element is a void element (self-closing by nature).
  fn is_void_element(tag_name: &str) -> bool {
    matches!(
      tag_name.to_ascii_lowercase().as_str(),
      "area"
        | "base"
        | "br"
        | "col"
        | "embed"
        | "hr"
        | "img"
        | "input"
        | "link"
        | "meta"
        | "source"
        | "track"
        | "wbr"
    )
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
<!-- Another comment -->"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn error_recovery_unclosed() {
    const HTML: &str = r#"<div>
  <p>Unclosed paragraph
</div>"#;

    assert_snapshot!(parse(HTML));
  }

  #[test]
  fn error_recovery_orphan_closing() {
    const HTML: &str = r#"<div>Content</div>
</span>
<p>More content</p>"#;

    assert_snapshot!(parse(HTML));
  }
}
