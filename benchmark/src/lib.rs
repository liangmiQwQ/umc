use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use oxc_allocator::Allocator;
use tokio::runtime::Runtime;
use umc_html_parser::CreateHtml;
use umc_parser::Parser;

fn bench_html(c: &mut Criterion) {
  // get html from https://www.wikipedia.org/
  let rt = Runtime::new().unwrap();
  let html = rt.block_on(async {
    reqwest::get("https://www.wikipedia.org/")
      .await
      .unwrap()
      .text()
      .await
      .unwrap()
  });

  c.bench_function("html parse", |b| {
    let allocator = Allocator::new();
    let parser = Parser::html(&allocator, &html);

    b.iter(|| {
      let result = parser.parse();
      black_box(result)
    });
  });
}

criterion_group!(benches, bench_html);
criterion_main!(benches);
