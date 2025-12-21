# AI Agent Guidelines

This document serves as a guide for AI agents and contributors working on the UMC (Unified Markup Compiler) project. It outlines the project structure, development workflow, and coding standards.

## Project Overview

UMC (Unified Markup Compiler) is a high-performance markup language compiler written in Rust. It leverages modern techniques like arena allocation (via `oxc_allocator`) to achieve speed and efficiency. The project is structured as a monorepo containing core utilities, language-specific parsers/ASTs, and benchmarks.

## Repository Structure

The workspace is organized into the following key implementation areas:

- **`core/`**: Shared infrastructure.
  - `umc_ast`: Core AST definitions and traits.
  - `umc_parser`: Base parser traits and common parsing utilities.
  - `umc_span`: Source span, location, and source text management.
- **`languages/`**: Language-specific implementations.
  - `html/umc_html_ast`: HTML AST definitions using `oxc_allocator`.
  - `html/umc_html_parser`: The HTML parser implementation.
- **`benchmark/`**: Benchmarking suite (using `criterion`).
- **`packages/`**: Node.js/NAPI bindings and other packages.

## Development Workflow

We use [`just`](https://github.com/casey/just) as a task runner. Please favor `just` commands over raw `cargo` or `pnpm` commands where possible.

### Common Tasks

- **Setup**: `just init` (installs dependencies and tools like `cargo-binstall`, `cargo-insta`, etc.)
- **Build**: `just build` (builds both Rust and JS parts)
- **Test**: `just test` (runs JS tests and `cargo test`)
- **Lint**: `just lint` (runs `cargo shear`, `clippy`, and `pnpm lint`)
- **Format**: `just fmt` (formats code using `rustfmt` and Prettier)
- **Benchmark**: `just bench` (runs `umc_benchmark`)
- **Prepare for PR**: `just ready` (runs lint, fix, test, and checks for git diffs)

## Coding Conventions

### Rust Patterns

1. **Allocator**: We use `oxc_allocator` for AST node allocation.
   - AST nodes should generally be allocated in an `Allocator` (Arena).
   - Use `&'a str` for strings within AST nodes, referring to the source or allocated in the arena.
   - Avoid `Box`, `Rc`, or `Arc` in the AST hot paths.

2. **Error Handling**:
   - Use `miette` for error reporting (via `oxc-miette` feature).
   - Parsers should be resilient, collecting errors rather than panicking or aborting immediately.

3. **Performance**:
   - Be mindful of allocations.
   - Use `peekable` iterators or byte-level parsing where appropriate.
   - Use `memchr` for fast byte searching.

### Tooling

- **Edition**: Rust 2024.
- **Linter**: Clippy (strict settings enforced via CI and `just lint`).
- **Formatter**: `rustfmt` (config in `.rustfmt.toml`).

## Architecture & Relationships

The UMC architecture separates the **core parsing infrastructure** from **language-specific implementations**. This design ensures consistency across different languages while allowing for flexibility.

### Core Crates

- **`umc_parser`**: Defines the foundational traits.
  - `LanguageParser`: The central trait that every new language must implement. It connects the configuration (`Option`), the output AST (`Result`), and the implementation logic (`Parser`).
  - `ParserImpl`: The trait that the actual parser struct must implement. It defines the standard `new` and `parse` methods.
  - `Parser`: A generic user-facing wrapper (e.g., `Parser::<Html>`) that provides a uniform API for all languages.

- **`umc_ast`**: Provides base AST traits and types that are shared, though most specific AST nodes live in their respective language crates.

- **`umc_span`**: Handles source code locations (`Span`) and source text management, heavily optimized for performance (forked from `oxc_span`).

### Language Crates

Each language (e.g., `html`) typically consists of two crates:

1. **`umc_<lang>_ast`**: Defines the AST nodes specific to that language. These nodes should be allocator-friendly (using `&'a str` and `oxc_allocator::Vec`).
2. **`umc_<lang>_parser`**: Implements the parsing logic.
   - It defines a marker struct (e.g., `struct Html;`) that implements `LanguageParser`.
   - It implements the actual parsing logic (e.g., `HtmlParserImpl`) satisfying `ParserImpl`.
   - It may provide extension traits (e.g., `CreateHtml`) for ergonomic usage (e.g., `Parser::html(...)`).

## Usage Guide

To use a parser (e.g., the HTML parser), you generally follow this pattern:

1. **Initialize an Allocator**: Memory is managed via an arena allocator for performance.
2. **Create a Parser**: Use the generic `Parser` or a language-specific helper.
3. **Parse**: Execute the parsing logic to get the AST and any errors.

### Example: Parsing HTML

```rust
use oxc_allocator::Allocator;
use umc_parser::Parser;
// Import the trait to enable the helper method `Parser::html`
use umc_html_parser::CreateHtml;

fn main() {
    // 1. Create the memory arena
    let allocator = Allocator::default();

    // 2. Define source text
    let source_text = "<div id='app'>Hello World</div>";

    // 3. Instantiate the parser using the helper method
    //    This creates a Parser::<Html> internally.
    let parser = Parser::html(&allocator, source_text);

    // 4. Run the parser
    let result = parser.parse();

    // 5. Handle results
    if result.errors.is_empty() {
        println!("Successfully parsed!");
        // `result.program` depends on the language, for HTML it is `Program<'a>` (Alias to `Vec<'a, Node<'a>>`)
        for node in result.program {
            println!("{:?}", node);
        }
    } else {
        for error in result.errors {
            println!("Error: {:?}", error);
        }
    }
}
```

## Adding a New Language

1. Create a new directory in `languages/<language_name>`.
2. Implement at least two crates:
   - `umc_<lang>_ast`: AST definitions.
   - `umc_<lang>_parser`: Parser logic.
3. Register new crates in `Cargo.toml` workspace members.
