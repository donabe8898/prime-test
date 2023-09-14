use criterion::{criterion_group, criterion_main, Criterion};

use mylib::ll;
use mylib::mr;
use rug::{Assign, Integer};

fn bm1(c: &mut Criterion) {
    c.bench_function("mr", |b| {
        b.iter(|| mr::is_prime_miller_rabin(Integer::from(133331), Integer::from(6)))
    });
}

fn bm2(c: &mut Criterion) {
    c.bench_function("ll", |b| {
        b.iter(|| ll::is_prime_lucal_lehmer(Integer::from(133331)))
    });
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);
