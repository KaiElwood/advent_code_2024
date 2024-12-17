
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// we don't want to benchmark how quickly we load from a file, instead we want to benchmark how quickly we can run and complete code

pub fn criterion_benchmark(c: &mut Criterion) {
  let rows = day4::get_text();
  c.bench_function("p1 solution", |b| {
    b.iter(|| day4::pt1(black_box(&rows)));
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
