//! A program to report all classes you used in a single HTML

use std::collections::HashSet;

use oxc_allocator::Allocator;
use umc_html_ast::Attribute;
use umc_html_parser::CreateHtml;
use umc_html_traverse::{TraverseHtml, traverse_program};
use umc_parser::Parser;
use umc_traverse::TraverseOperate;

/// HTML string
const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tailwind Example</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-50">
    <div class="max-w-2xl mx-auto p-8">
        <header class="text-center mb-12">
            <h1 class="text-4xl font-bold text-blue-600 mb-4">Welcome</h1>
            <p class="text-gray-600">A simple page with Tailwind CSS</p>
        </header>
        
        <main class="space-y-8">
            <div class="bg-white p-6 rounded-lg shadow">
                <h2 class="text-2xl font-bold text-gray-800 mb-3">About</h2>
                <p class="text-gray-600">This page uses Tailwind CSS for styling.</p>
                <p class="text-gray-600 mt-2">No custom CSS needed!</p>
            </div>
            
            <div class="bg-blue-50 p-6 rounded-lg border-l-4 border-blue-500">
                <h3 class="text-xl font-semibold text-blue-800">Features</h3>
                <ul class="mt-3 space-y-2">
                    <li class="flex items-start">
                        <span class="text-blue-500 mr-2">✓</span>
                        <span>Responsive design</span>
                    </li>
                    <li class="flex items-start">
                        <span class="text-blue-500 mr-2">✓</span>
                        <span>Easy to use</span>
                    </li>
                    <li class="flex items-start">
                        <span class="text-blue-500 mr-2">✓</span>
                        <span>No custom CSS</span>
                    </li>
                </ul>
            </div>
            
            <div class="text-center">
                <button class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-6 rounded-lg transition">
                    Learn More
                </button>
            </div>
        </main>
        
        <footer class="mt-12 pt-6 border-t border-gray-200 text-center text-gray-500 text-sm">
            <p>Simple Tailwind CSS Example © 2023</p>
        </footer>
    </div>
</body>
</html>"#;

fn main() {
  let allocator = Allocator::new();

  let parser = Parser::html(&allocator, HTML);
  let program = parser.parse().program;

  let mut collector = Collector::default();
  traverse_program(&program, &mut collector);

  for class in collector.result {
    println!("{class}");
  }
}

#[derive(Default)]
struct Collector<'a> {
  result: HashSet<&'a str>,
}

impl<'a> TraverseHtml<'a> for Collector<'a> {
  fn enter_attribute(&mut self, attribute: &Attribute<'a>) -> TraverseOperate {
    if attribute.key.value == "class"
      && let Some(value) = &attribute.value
    {
      self.result.extend(value.value.split_whitespace());
    }

    TraverseOperate::Skip
  }
}
