use criterion::{criterion_group, criterion_main, Criterion};

use mylib::test;

// use rug::{Assign, Integer};

fn bm1(c: &mut Criterion) {
    c.bench_function("mr+mr", |b| b.iter(|| test::mr_mr_bench(256)));
}

criterion_group!(benches, bm1);
criterion_main!(benches);
