use super::kind::Html5Kind;

#[derive(Debug, PartialEq)]
pub struct Html5Token {
  pub kind: Html5Kind,
  pub start: usize,
  pub end: usize,
  pub value: Html5TokenValue,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Html5TokenValue {
  #[default]
  None,
  String(String),
}
