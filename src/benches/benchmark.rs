use criterion::{criterion_group, criterion_main, Criterion};

use mylib::test;

// use rug::{Assign, Integer};

fn bm1(c: &mut Criterion) {
    c.bench_function("mr+mr", |b| b.iter(|| test::mr_mr_bench(512)));
}

fn bm2(c: &mut Criterion) {
    c.bench_function("mr+eel", |b| b.iter(|| test::mr_eel_bench(64)));
}
//fn bm2(c: &mut Criterion) {
//    c.bench_function("mr+mr", |b| b.iter(|| test::mr_bench(512)));
//}

criterion_group!(benches, bm2);
criterion_main!(benches);
