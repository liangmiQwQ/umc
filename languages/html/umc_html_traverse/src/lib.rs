use umc_html_ast::{
  Attribute, AttributeKey, AttributeValue, Comment, Doctype, Element, Node, Program, Script, Text,
};
use umc_traverse::TraverseOperate;

#[expect(unused_variables)]
pub trait TraverseHtml<'a> {
  fn enter_program(&mut self, program: &Program<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_node(&mut self, node: &Node<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_element(&mut self, element: &Element<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_doctype(&mut self, doctype: &Doctype<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_comment(&mut self, comment: &Comment<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_text(&mut self, text: &Text<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_script(&mut self, script: &Script<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute(&mut self, attribute: &Attribute<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_key(&mut self, attribute_key: &AttributeKey<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_value(&mut self, attribute_value: &AttributeValue<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn exit_program(&mut self, program: &Program<'a>) {}
  fn exit_node(&mut self, node: &Node<'a>) {}
  fn exit_element(&mut self, element: &Element<'a>) {}
  fn exit_doctype(&mut self, doctype: &Doctype<'a>) {}
  fn exit_comment(&mut self, comment: &Comment<'a>) {}
  fn exit_text(&mut self, text: &Text<'a>) {}
  fn exit_script(&mut self, script: &Script<'a>) {}
  fn exit_attribute(&mut self, attribute: &Attribute<'a>) {}
  fn exit_attribute_key(&mut self, attribute_key: &AttributeKey<'a>) {}
  fn exit_attribute_value(&mut self, attribute_value: &AttributeValue<'a>) {}
}

pub fn traverse_program<'a>(program: &Program<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_program(program) != TraverseOperate::Skip {
    for node in program {
      traverse_node(node, traverse);
    }
    traverse.exit_program(program);
  }
}

pub fn traverse_node<'a>(node: &Node<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_node(node) != TraverseOperate::Skip {
    match node {
      Node::Doctype(doctype) => traverse_doctype(doctype, traverse),
      Node::Element(element) => traverse_element(element, traverse),
      Node::Text(text) => traverse_text(text, traverse),
      Node::Comment(comment) => traverse_comment(comment, traverse),
      Node::Script(script) => traverse_script(script, traverse),
    }
    traverse.exit_node(node);
  }
}

pub fn traverse_doctype<'a>(doctype: &Doctype<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_doctype(doctype) != TraverseOperate::Skip {
    for attribute in &doctype.attributes {
      traverse_attribute(attribute, traverse);
    }
    traverse.exit_doctype(doctype);
  }
}

pub fn traverse_element<'a>(element: &Element<'a>, traverse: &mut impl TraverseHtml<'a>) {
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

pub fn traverse_comment<'a>(comment: &Comment<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_comment(comment) != TraverseOperate::Skip {
    traverse.exit_comment(comment);
  }
}

pub fn traverse_text<'a>(text: &Text<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_text(text) != TraverseOperate::Skip {
    traverse.exit_text(text);
  }
}

pub fn traverse_attribute<'a>(attribute: &Attribute<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_attribute(attribute) != TraverseOperate::Skip {
    traverse_attribute_key(&attribute.key, traverse);
    if let Some(value) = &attribute.value {
      traverse_attribute_value(value, traverse);
    }
    traverse.exit_attribute(attribute);
  }
}

pub fn traverse_attribute_key<'a>(
  attribute_key: &AttributeKey<'a>,
  traverse: &mut impl TraverseHtml<'a>,
) {
  if traverse.enter_attribute_key(attribute_key) != TraverseOperate::Skip {
    traverse.exit_attribute_key(attribute_key);
  }
}

pub fn traverse_attribute_value<'a>(
  attribute_value: &AttributeValue<'a>,
  traverse: &mut impl TraverseHtml<'a>,
) {
  if traverse.enter_attribute_value(attribute_value) != TraverseOperate::Skip {
    traverse.exit_attribute_value(attribute_value);
  }
}

/// Traverse a script node without traversing the JavaScript AST.
/// Per requirement, we only traverse the HTML attributes, not the JS nodes.
pub fn traverse_script<'a>(script: &Script<'a>, traverse: &mut impl TraverseHtml<'a>) {
  if traverse.enter_script(script) != TraverseOperate::Skip {
    for attribute in &script.attributes {
      traverse_attribute(attribute, traverse);
    }
    // Note: We intentionally do NOT traverse the JavaScript AST nodes
    traverse.exit_script(script);
  }
}

#[expect(unused_variables)]
pub trait TraverseHtmlMut<'a> {
  fn enter_program(&mut self, program: &mut Program<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_node(&mut self, node: &mut Node<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_element(&mut self, element: &mut Element<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_doctype(&mut self, doctype: &mut Doctype<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_comment(&mut self, comment: &mut Comment<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_text(&mut self, text: &mut Text<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_script(&mut self, script: &mut Script<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute(&mut self, attribute: &mut Attribute<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_key(&mut self, attribute_key: &mut AttributeKey<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn enter_attribute_value(&mut self, attribute_value: &mut AttributeValue<'a>) -> TraverseOperate {
    TraverseOperate::Continue
  }
  fn exit_program(&mut self, program: &mut Program<'a>) {}
  fn exit_node(&mut self, node: &mut Node<'a>) {}
  fn exit_element(&mut self, element: &mut Element<'a>) {}
  fn exit_doctype(&mut self, doctype: &mut Doctype<'a>) {}
  fn exit_comment(&mut self, comment: &mut Comment<'a>) {}
  fn exit_text(&mut self, text: &mut Text<'a>) {}
  fn exit_script(&mut self, script: &mut Script<'a>) {}
  fn exit_attribute(&mut self, attribute: &mut Attribute<'a>) {}
  fn exit_attribute_key(&mut self, attribute_key: &mut AttributeKey<'a>) {}
  fn exit_attribute_value(&mut self, attribute_value: &mut AttributeValue<'a>) {}
}

pub fn traverse_program_mut<'a>(
  program: &mut Program<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
  if traverse.enter_program(program) != TraverseOperate::Skip {
    for node in &mut *program {
      traverse_node_mut(node, traverse);
    }
    traverse.exit_program(program);
  }
}

pub fn traverse_node_mut<'a>(node: &mut Node<'a>, traverse: &mut impl TraverseHtmlMut<'a>) {
  if traverse.enter_node(node) != TraverseOperate::Skip {
    match node {
      Node::Doctype(doctype) => traverse_doctype_mut(doctype, traverse),
      Node::Element(element) => traverse_element_mut(element, traverse),
      Node::Text(text) => traverse_text_mut(text, traverse),
      Node::Comment(comment) => traverse_comment_mut(comment, traverse),
      Node::Script(script) => traverse_script_mut(script, traverse),
    }
    traverse.exit_node(node);
  }
}

pub fn traverse_doctype_mut<'a>(
  doctype: &mut Doctype<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
  if traverse.enter_doctype(doctype) != TraverseOperate::Skip {
    for attribute in &mut doctype.attributes {
      traverse_attribute_mut(attribute, traverse);
    }
    traverse.exit_doctype(doctype);
  }
}

pub fn traverse_element_mut<'a>(
  element: &mut Element<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
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

pub fn traverse_comment_mut<'a>(
  comment: &mut Comment<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
  if traverse.enter_comment(comment) != TraverseOperate::Skip {
    traverse.exit_comment(comment);
  }
}

pub fn traverse_text_mut<'a>(text: &mut Text<'a>, traverse: &mut impl TraverseHtmlMut<'a>) {
  if traverse.enter_text(text) != TraverseOperate::Skip {
    traverse.exit_text(text);
  }
}

pub fn traverse_attribute_mut<'a>(
  attribute: &mut Attribute<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
  if traverse.enter_attribute(attribute) != TraverseOperate::Skip {
    traverse_attribute_key_mut(&mut attribute.key, traverse);
    if let Some(value) = &mut attribute.value {
      traverse_attribute_value_mut(value, traverse);
    }
    traverse.exit_attribute(attribute);
  }
}

pub fn traverse_attribute_key_mut<'a>(
  attribute_key: &mut AttributeKey<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
  if traverse.enter_attribute_key(attribute_key) != TraverseOperate::Skip {
    traverse.exit_attribute_key(attribute_key);
  }
}

pub fn traverse_attribute_value_mut<'a>(
  attribute_value: &mut AttributeValue<'a>,
  traverse: &mut impl TraverseHtmlMut<'a>,
) {
  if traverse.enter_attribute_value(attribute_value) != TraverseOperate::Skip {
    traverse.exit_attribute_value(attribute_value);
  }
}

/// Traverse a script node mutably without traversing the JavaScript AST.
/// Per requirement, we only traverse the HTML attributes, not the JS nodes.
pub fn traverse_script_mut<'a>(script: &mut Script<'a>, traverse: &mut impl TraverseHtmlMut<'a>) {
  if traverse.enter_script(script) != TraverseOperate::Skip {
    for attribute in &mut script.attributes {
      traverse_attribute_mut(attribute, traverse);
    }
    // Note: We intentionally do NOT traverse the JavaScript AST nodes
    traverse.exit_script(script);
  }
}
