# umc_parser

> Core parser infrastructure for the Universal Markup-language Compiler (UMC).

This crate provides the foundational traits and types for implementing language-specific parsers within the UMC ecosystem. It defines a generic parser framework that can be adapted for different markup languages (HTML, XML, etc.).

## Features

- **Language Agnostic**: Defines the `LanguageParser` trait to support implementing parsers for various languages.
- **Arena Allocation**: Built with `oxc_allocator` for high-performance memory management.
- **Error Handling**: Integrated with `oxc_diagnostics` for robust error reporting.

## Usage

Implement the `LanguageParser` trait for your specific language:

```rust
struct Html;

impl LanguageParser for Html {
  type Result<'a> = oxc_allocator::Vec<'a, Node<'a>>;
  type Option = HtmlParserOption;
  type Parser<'a> = HtmlParserImpl<'a>;
}
```

Then you can use the generic `Parser` to parse your language:

```rust
let allocator = Allocator::default();
let parser = Parser::<Html>::new(&allocator, "<html></html>");
let result = parser.parse();
```
