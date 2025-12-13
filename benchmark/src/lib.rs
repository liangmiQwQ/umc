use criterion::{Criterion, criterion_group, criterion_main};

async fn bench_html(c: &mut Criterion) {
  // get html from https://www.wikipedia.org/
  let html = reqwest::get("https://www.wikipedia.org/")
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  c.bench_function("html parse", |b| {});
}

fn bench() {
  let allocator = Allocator::new();
}

criterion_group!(benches, bench_html);
criterion_main!(benches);
