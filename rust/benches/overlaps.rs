use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, ValueFormatter},
    Criterion,
};
use mashhap::{
    chaining,
    hash::{fnv_1a, murmurhash3, seahash},
    MashHap,
};

struct Overlaps;
impl Measurement for Overlaps {
    type Intermediate = usize;
    type Value = usize;

    fn start(&self) -> Self::Intermediate {
        0
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        i
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &OverlapFormatter
    }
}

struct OverlapFormatter;
impl ValueFormatter for OverlapFormatter {
    fn scale_values(&self, _: f64, _: &mut [f64]) -> &'static str {
        "overlaps"
    }

    fn scale_throughputs(&self, _: f64, _: &criterion::Throughput, _: &mut [f64]) -> &'static str {
        "overlaps"
    }

    fn scale_for_machines(&self, _: &mut [f64]) -> &'static str {
        "overlaps"
    }
}

fn bench_fn(c: &mut Criterion<Overlaps>) {
    let words: Vec<_> = include_str!("../randomwords.txt").split(' ').collect();

    c.bench_function("overlaps FNV-1a open addressing", |b| {
        b.iter_custom(|iters| {
            let mut overlaps = 0;
            for _ in 0..iters {
                let mut map = MashHap::new(fnv_1a);
                for word in &words {
                    map.set(word, 5);
                }
                overlaps += map.overlaps();
            }
            overlaps / words.len()
        })
    });

    c.bench_function("overlaps SeaHash open addressing", |b| {
        b.iter_custom(|iters| {
            let mut overlaps = 0;
            for _ in 0..iters {
                let mut map = MashHap::new(seahash);
                for word in &words {
                    map.set(word, 5);
                }
                overlaps += map.overlaps();
            }
            overlaps / words.len()
        })
    });

    c.bench_function("overlaps MurmurHash 3 open addressing", |b| {
        b.iter_custom(|iters| {
            let mut overlaps = 0;
            for _ in 0..iters {
                let mut map = MashHap::new(murmurhash3);
                for word in &words {
                    map.set(word, 5);
                }
                overlaps += map.overlaps();
            }
            overlaps / words.len()
        })
    });

    c.bench_function("overlaps FNV-1a chaining", |b| {
        b.iter_custom(|iters| {
            let mut overlaps = 0;
            for _ in 0..iters {
                let mut map = chaining::MashHap::new(fnv_1a);
                for word in &words {
                    map.set(word, 5);
                }
                overlaps += map.overlaps();
            }
            overlaps / words.len()
        })
    });

    c.bench_function("overlaps SeaHash chaining", |b| {
        b.iter_custom(|iters| {
            let mut overlaps = 0;
            for _ in 0..iters {
                let mut map = chaining::MashHap::new(seahash);
                for word in &words {
                    map.set(word, 5);
                }
                overlaps += map.overlaps();
            }
            overlaps / words.len()
        })
    });

    c.bench_function("overlaps MurmurHash 3 chaining", |b| {
        b.iter_custom(|iters| {
            let mut overlaps = 0;
            for _ in 0..iters {
                let mut map = chaining::MashHap::new(murmurhash3);
                for word in &words {
                    map.set(word, 5);
                }
                overlaps += map.overlaps();
            }
            overlaps / words.len()
        })
    });
}

fn alternate_measurement() -> Criterion<Overlaps> {
    Criterion::default().with_measurement(Overlaps)
}

criterion_group! {
    name = benches;
    config = alternate_measurement();
    targets = bench_fn
}
criterion_main!(benches);
