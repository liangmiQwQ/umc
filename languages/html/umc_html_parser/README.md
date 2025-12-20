# umc_html_parser

> HTML parser implementation for the Universal Markup-language Compiler (UMC).

This crate provides a high-performance HTML parser that produces an AST defined in `umc_html_ast`. It is designed to be fast, error-tolerant, and capable of handling embedded languages like JavaScript and CSS.

## Features

- **Fast**: Built on top of `umc_parser` and `oxc_allocator`.
- **Embedded Language Support**: Can optionally parse content inside `<script>` and `<style>` tags using specific parsers (e.g., `oxc_parser` for JS).
- **Error Tolerant**: Collects errors without stopping parsing, suitable for IDEs and tools.

## Usage

```rust
use umc_html_parser::CreateHtml;
use umc_parser::Parser;
use oxc_allocator::Allocator;

let allocator = Allocator::default();
let source = "<html><body><h1>Hello World</h1></body></html>";

// Create and run the parser
let parser = Parser::html(&allocator, source);
let result = parser.parse();

// Access the AST
for node in result.program {
    println!("{:?}", node);
}

// Check for errors
if !result.errors.is_empty() {
    for error in result.errors {
        println!("Error: {}", error);
    }
}
```
