# umc_html_traverse

This crate provides a set of traits and functions for traversing HTML ASTs.

Require use with `umc_traverse` crate

## Usage

```rust
use umc_html_traverse::{TraverseHtml, TraverseHtmlMut};
use umc_traverse::TraverseOperate;

struct MyVisitor;

impl TraverseHtml for MyVisitor {
    fn traverse_element(&mut self, element: &Element) -> TraverseOperate {
        // Do something with the element
        TraverseOperate::Continue
    }
}
```
