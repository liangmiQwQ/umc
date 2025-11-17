use crate::ParserOptions;

impl ParserOptions {
  pub fn default_from_filename(filename: &str) -> Self {
    if filename.ends_with(".html") {
      Self::Html5 {}
    } else {
      panic!("Not Support Language")
    }
  }
}
