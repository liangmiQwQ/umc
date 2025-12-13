use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use oxc_allocator::Allocator;
use std::hint::black_box;
use tokio::runtime::Runtime;
use umc_html_parser::CreateHtml;
use umc_parser::Parser;

fn bench_html(c: &mut Criterion) {
  let mut group = c.benchmark_group("html_parse_by_size");

  // get html from https://www.wikipedia.org/
  let rt = Runtime::new().unwrap();

  let samples = [
    ("small", "<div>Hello</div>".repeat(10)),
    ("medium", "<div>Hello</div>".repeat(100)),
    ("large", "<div>Hello</div>".repeat(1000)),
    ("superlarge", "<div>Hello</div>".repeat(2000)),
    (
      "wikipedia",
      rt.block_on(async {
        reqwest::get("https://www.wikipedia.org/")
          .await
          .unwrap()
          .text()
          .await
          .unwrap()
      }),
    ),
  ];

  for (name, html) in samples.iter() {
    group.throughput(Throughput::Bytes(html.len() as u64));

    group.bench_with_input(BenchmarkId::from_parameter(name), html, |b, html| {
      b.iter(|| {
        let allocator = Allocator::new();
        let parser = Parser::html(&allocator, black_box(html));
        black_box(parser.parse());
      });
    });
  }

  group.finish();
}

criterion_group!(benches, bench_html);
criterion_main!(benches);
