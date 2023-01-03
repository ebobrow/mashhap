use criterion::{criterion_group, criterion_main, Criterion};
use mashhap::MashHap;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 10000 'a's", |b| {
        b.iter(|| {
            let mut map = MashHap::new();
            for i in 0..10000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });

    c.bench_function("insert 10000 'a's without resizing", |b| {
        b.iter(|| {
            let mut map = MashHap::with_capacity(10000);
            for i in 0..10000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
