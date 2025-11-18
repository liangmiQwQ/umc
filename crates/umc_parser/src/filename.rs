use crate::{ParserOptions, html::HtmlParserOption};

impl ParserOptions {
  pub fn default_from_filename(filename: &str) -> Self {
    let ext = filename.split(".").last().unwrap_or(filename);

    match ext {
      "html" | "htm" => ParserOptions::Html5(HtmlParserOption::default()),
      _ => panic!("Unknown file extension {} in file {}", ext, filename),
    }
  }
}
