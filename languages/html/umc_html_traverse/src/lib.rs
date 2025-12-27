use umc_html_ast::{
  Attribute, AttributeKey, AttributeValue, Comment, Doctype, Element, Node, Program, Text,
};
use umc_traverse::TraverseOperate;

#[expect(unused_variables)]
pub trait TraverseHtml {
  fn enter_program(&self, program: &Program) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_node(&self, node: &Node) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_element(&self, element: &Element) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_doctype(&self, doctype: &Doctype) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_comment(&self, comment: &Comment) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_text(&self, text: &Text) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute(&self, attribute: &Attribute) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_key(&self, attribute_key: &AttributeKey) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_value(&self, attribute_value: &AttributeValue) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn exit_program(&self, program: &Program) {}
  fn exit_node(&self, node: &Node) {}
  fn exit_element(&self, element: &Element) {}
  fn exit_doctype(&self, doctype: &Doctype) {}
  fn exit_comment(&self, comment: &Comment) {}
  fn exit_text(&self, text: &Text) {}
  fn exit_attribute(&self, attribute: &Attribute) {}
  fn exit_attribute_key(&self, attribute_key: &AttributeKey) {}
  fn exit_attribute_value(&self, attribute_value: &AttributeValue) {}
}

pub fn traverse_program(program: &Program, traverse: &impl TraverseHtml) {
  if traverse.enter_program(program) != TraverseOperate::Skip {
    for node in program {
      traverse_node(node, traverse);
    }
    traverse.exit_program(program);
  }
}

pub fn traverse_node(node: &Node, traverse: &impl TraverseHtml) {
  if traverse.enter_node(node) != TraverseOperate::Skip {
    match node {
      Node::Doctype(doctype) => traverse_doctype(doctype, traverse),
      Node::Element(element) => traverse_element(element, traverse),
      Node::Text(text) => traverse_text(text, traverse),
      Node::Comment(comment) => traverse_comment(comment, traverse),
    }
    traverse.exit_node(node);
  }
}

pub fn traverse_doctype(doctype: &Doctype, traverse: &impl TraverseHtml) {
  if traverse.enter_doctype(doctype) != TraverseOperate::Skip {
    for attribute in &doctype.attributes {
      traverse_attribute(attribute, traverse);
    }
    traverse.exit_doctype(doctype);
  }
}

pub fn traverse_element(element: &Element, traverse: &impl TraverseHtml) {
  if traverse.enter_element(element) != TraverseOperate::Skip {
    for attribute in &element.attributes {
      traverse_attribute(attribute, traverse);
    }
    for node in &element.children {
      traverse_node(node, traverse);
    }
    traverse.exit_element(element);
  }
}

pub fn traverse_comment(comment: &Comment, traverse: &impl TraverseHtml) {
  if traverse.enter_comment(comment) != TraverseOperate::Skip {
    traverse.exit_comment(comment);
  }
}

pub fn traverse_text(text: &Text, traverse: &impl TraverseHtml) {
  if traverse.enter_text(text) != TraverseOperate::Skip {
    traverse.exit_text(text);
  }
}

pub fn traverse_attribute(attribute: &Attribute, traverse: &impl TraverseHtml) {
  if traverse.enter_attribute(attribute) != TraverseOperate::Skip {
    traverse_attribute_key(&attribute.key, traverse);
    if let Some(value) = &attribute.value {
      traverse_attribute_value(value, traverse);
    }
    traverse.exit_attribute(attribute);
  }
}

pub fn traverse_attribute_key(attribute_key: &AttributeKey, traverse: &impl TraverseHtml) {
  if traverse.enter_attribute_key(attribute_key) != TraverseOperate::Skip {
    traverse.exit_attribute_key(attribute_key);
  }
}

pub fn traverse_attribute_value(attribute_value: &AttributeValue, traverse: &impl TraverseHtml) {
  if traverse.enter_attribute_value(attribute_value) != TraverseOperate::Skip {
    traverse.exit_attribute_value(attribute_value);
  }
}

#[expect(unused_variables)]
pub trait TraverseHtmlMut {
  fn enter_program(&self, program: &mut Program) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_node(&self, node: &mut Node) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_element(&self, element: &mut Element) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_doctype(&self, doctype: &mut Doctype) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_comment(&self, comment: &mut Comment) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_text(&self, text: &mut Text) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute(&self, attribute: &mut Attribute) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_key(&self, attribute_key: &mut AttributeKey) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_value(&self, attribute_value: &mut AttributeValue) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn exit_program(&self, program: &mut Program) {}
  fn exit_node(&self, node: &mut Node) {}
  fn exit_element(&self, element: &mut Element) {}
  fn exit_doctype(&self, doctype: &mut Doctype) {}
  fn exit_comment(&self, comment: &mut Comment) {}
  fn exit_text(&self, text: &mut Text) {}
  fn exit_attribute(&self, attribute: &mut Attribute) {}
  fn exit_attribute_key(&self, attribute_key: &mut AttributeKey) {}
  fn exit_attribute_value(&self, attribute_value: &mut AttributeValue) {}
}

pub fn traverse_program_mut(program: &mut Program, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_program(program) != TraverseOperate::Skip {
    for node in &mut *program {
      traverse_node_mut(node, traverse);
    }
    traverse.exit_program(program);
  }
}

pub fn traverse_node_mut(node: &mut Node, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_node(node) != TraverseOperate::Skip {
    match node {
      Node::Doctype(doctype) => traverse_doctype_mut(doctype, traverse),
      Node::Element(element) => traverse_element_mut(element, traverse),
      Node::Text(text) => traverse_text_mut(text, traverse),
      Node::Comment(comment) => traverse_comment_mut(comment, traverse),
    }
    traverse.exit_node(node);
  }
}

pub fn traverse_doctype_mut(doctype: &mut Doctype, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_doctype(doctype) != TraverseOperate::Skip {
    for attribute in &mut doctype.attributes {
      traverse_attribute_mut(attribute, traverse);
    }
    traverse.exit_doctype(doctype);
  }
}

pub fn traverse_element_mut(element: &mut Element, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_element(element) != TraverseOperate::Skip {
    for attribute in &mut element.attributes {
      traverse_attribute_mut(attribute, traverse);
    }
    for node in &mut element.children {
      traverse_node_mut(node, traverse);
    }
    traverse.exit_element(element);
  }
}

pub fn traverse_comment_mut(comment: &mut Comment, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_comment(comment) != TraverseOperate::Skip {
    traverse.exit_comment(comment);
  }
}

pub fn traverse_text_mut(text: &mut Text, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_text(text) != TraverseOperate::Skip {
    traverse.exit_text(text);
  }
}

pub fn traverse_attribute_mut(attribute: &mut Attribute, traverse: &impl TraverseHtmlMut) {
  if traverse.enter_attribute(attribute) != TraverseOperate::Skip {
    traverse_attribute_key_mut(&mut attribute.key, traverse);
    if let Some(value) = &mut attribute.value {
      traverse_attribute_value_mut(value, traverse);
    }
    traverse.exit_attribute(attribute);
  }
}

pub fn traverse_attribute_key_mut(
  attribute_key: &mut AttributeKey,
  traverse: &impl TraverseHtmlMut,
) {
  if traverse.enter_attribute_key(attribute_key) != TraverseOperate::Skip {
    traverse.exit_attribute_key(attribute_key);
  }
}

pub fn traverse_attribute_value_mut(
  attribute_value: &mut AttributeValue,
  traverse: &impl TraverseHtmlMut,
) {
  if traverse.enter_attribute_value(attribute_value) != TraverseOperate::Skip {
    traverse.exit_attribute_value(attribute_value);
  }
}
