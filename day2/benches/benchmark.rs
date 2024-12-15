use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::{get_text, is_safe, is_safe_dampened, is_safe_dampened_2};

pub fn bench_is_safe(c: &mut Criterion) {
    let test_data = get_text();
    c.bench_function("is_safe", |b| {
        b.iter(|| {
            for report in test_data.iter() {
                let _ = is_safe(black_box(report));
            }
        })
    });
}

pub fn bench_is_safe_dampened(c: &mut Criterion) {
    let test_data = get_text();
    c.bench_function("is_safe_dampened", |b| {
        b.iter(|| {
            for report in test_data.iter() {
                let _ = is_safe_dampened(black_box(report));
            }
        })
    });
}

pub fn bench_is_safe_dampened_2(c: &mut Criterion) {
    let test_data = get_text();
    c.bench_function("is_safe_dampened_2", |b| {
        b.iter(|| {
            for report in test_data.iter() {
                let _ = is_safe_dampened_2(black_box(report));
            }
        })
    });
}

criterion_group!(benches, bench_is_safe, bench_is_safe_dampened, bench_is_safe_dampened_2);
criterion_main!(benches);