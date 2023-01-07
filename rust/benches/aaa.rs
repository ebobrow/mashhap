use criterion::{criterion_group, criterion_main, Criterion};
use mashhap::{
    hash::{fnv_1a, seahash},
    MashHap,
};

fn criterion_benchmark(c: &mut Criterion) {
    // TODO: separate file or function for each hash function
    c.bench_function("insert 1000 'a's FNV-1a", |b| {
        b.iter(|| {
            let mut map = MashHap::new(fnv_1a);
            for i in 0..1000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });

    c.bench_function("insert 1000 'a's SeaHash", |b| {
        b.iter(|| {
            let mut map = MashHap::new(seahash);
            for i in 0..1000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });

    let words: Vec<_> = include_str!("../randomwords.txt").split(' ').collect();
    let words2: Vec<_> = include_str!("../randomwords2.txt").split(' ').collect();

    // TODO: try increments up to 10000 to see when SeaHash overtakes FNV-1a
    c.bench_function("set FNV-1a", |b| {
        b.iter(|| {
            let mut map = MashHap::new(fnv_1a);
            for word in &words {
                map.set(word, 5);
            }
        })
    });

    c.bench_function("set SeaHash", |b| {
        b.iter(|| {
            let mut map = MashHap::new(seahash);
            for word in &words {
                map.set(word, 5);
            }
        })
    });

    let mut preset_map_fnv = MashHap::new(fnv_1a);
    for word in &words {
        preset_map_fnv.set(word, 5);
    }

    c.bench_function("get FNV-1a", |b| {
        b.iter(|| {
            for word in &words {
                preset_map_fnv.get(word);
            }
            for word in &words2 {
                preset_map_fnv.get(word);
            }
        })
    });

    let mut preset_map_seahash = MashHap::new(seahash);
    for word in &words {
        preset_map_seahash.set(word, 5);
    }

    c.bench_function("get SeaHash", |b| {
        b.iter(|| {
            for word in &words {
                preset_map_seahash.get(word);
            }
            for word in &words2 {
                preset_map_seahash.get(word);
            }
        })
    });

    c.bench_function("delete FNV-1a", |b| {
        b.iter(|| {
            for word in &words {
                preset_map_fnv.delete(word);
            }
            for word in &words2 {
                preset_map_fnv.delete(word);
            }
        })
    });

    c.bench_function("delete SeaHash", |b| {
        b.iter(|| {
            for word in &words {
                preset_map_seahash.delete(word);
            }
            for word in &words2 {
                preset_map_seahash.delete(word);
            }
        })
    });

    c.bench_function("all three FNV-1a", |b| {
        b.iter(|| {
            let mut map = MashHap::new(fnv_1a);
            for word in &words {
                map.set(word, 5);
            }
            for word in &words {
                map.get(word);
            }
            for word in &words {
                map.delete(word);
            }
        })
    });

    c.bench_function("all three SeaHash", |b| {
        b.iter(|| {
            let mut map = MashHap::new(seahash);
            for word in &words {
                map.set(word, 5);
            }
            for word in &words {
                map.get(word);
            }
            for word in &words {
                map.delete(word);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
