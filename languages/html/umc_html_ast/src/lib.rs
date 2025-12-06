use umc_span::Span;

pub enum Node {
  Doctype(Doctype),
  Element(Element),
  Text(Text),
  Comment(Comment),
}

pub struct Doctype {
  pub span: Span,
  pub attributes: Vec<Attribute>,
}

pub struct Element {
  pub span: Span,
  pub tag_name: String,
  pub attributes: Vec<Attribute>,
  pub children: Vec<Node>,
}

pub struct Text {
  pub span: Span,
  pub value: String,
}

pub struct Comment {
  pub span: Span,
  pub bogus: bool,
  pub value: String,
}

pub struct Attribute {
  pub key: String,
  pub value: String,
}
