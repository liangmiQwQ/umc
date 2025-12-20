# umc_span

> A fork from [oxc_span](https://github.com/oxc-project/oxc/tree/main/crates/oxc_span), Source span definitions and utilities for UMC.

This crate provides the `Span` type and related utilities for tracking source code positions and ranges. It is a fork of `oxc_span` tailored for UMC, removing dependencies on `estree` and `oxc_ast` while keeping the core span functionality.

## Features

- **Compact Representation**: Spans are represented by `u32` start and end offsets.
- **Performance**: Optimized for size and speed, aligning on 64-bit platforms.
- **Utilities**: Provides methods for merging, expanding, shrinking, and checking containment of spans.
- **Miette Integration**: Implements `miette::SourceSpan` for easy error reporting.

## Usage

```rust
use umc_span::Span;

let span = Span::new(0, 5);
assert_eq!(span.size(), 5);
```
