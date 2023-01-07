use criterion::{criterion_group, criterion_main, Criterion};
use mashhap::{
    hash::{fnv_1a, seahash},
    MashHap,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 10000 'a's FNV-1a", |b| {
        b.iter(|| {
            let mut map = MashHap::new(fnv_1a);
            for i in 0..10000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });

    c.bench_function("insert 10000 'a's SeaHash", |b| {
        b.iter(|| {
            let mut map = MashHap::new(seahash);
            for i in 0..10000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });
    // TODO: Test get, delete methods

    // c.bench_function("insert 10000 'a's without resizing", |b| {
    //     b.iter(|| {
    //         let mut map = MashHap::with_capacity(10000, fnv_1a);
    //         for i in 0..10000 {
    //             map.set("a".repeat(i + 1), i);
    //         }
    //     })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
