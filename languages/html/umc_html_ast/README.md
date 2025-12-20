# umc_html_ast

> HTML Abstract Syntax Tree (AST) node definitions for UMC.

This crate defines the AST node types used to represent parsed HTML documents. It includes definitions for `Doctype`, `Element`, `Text`, and `Comment` nodes.

## Features

- **Arena Allocated**: All AST nodes are designed to be allocated in an arena (using `oxc_allocator`) for high performance and efficient memory cleanup.
- **Zero-Copy**: String data uses `&'a str` references to the original source text where possible.
- **Comprehensive**: Covers standard HTML node types including attributes.

## Structure

- `Node`: Enum wrapping all possible HTML node types.
- `Element`: Represents an HTML tag with attributes and children.
- `Text`: Represents text content.
- `Comment`: Represents HTML comments.
- `Doctype`: Represents the document type declaration.
