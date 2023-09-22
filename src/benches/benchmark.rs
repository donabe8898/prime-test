use criterion::{criterion_group, criterion_main, Criterion};

use mylib::test;

use rug::{Assign, Integer};

fn bm1(c: &mut Criterion) {
    c.bench_function("mr", |b| b.iter(|| test::mr_bench(2048)));
}

fn bm2(c: &mut Criterion) {
    c.bench_function("ll", |b| b.iter(|| test::llr_bench(2048)));
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);
