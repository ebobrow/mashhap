use criterion::{criterion_group, criterion_main, Criterion};
use mashhap::{
    hash::{fnv_1a, murmurhash3, seahash, HashFn},
    MashHap,
};

fn bench_suite(c: &mut Criterion, f: HashFn, name: &str) {
    // TODO: separate file or function for each hash function
    c.bench_function(&format!("insert 1000 'a's {}", name), |b| {
        b.iter(|| {
            let mut map = MashHap::new(f);
            for i in 0..1000 {
                map.set("a".repeat(i + 1), i);
            }
        })
    });

    let words: Vec<_> = include_str!("../randomwords.txt").split(' ').collect();
    let words2: Vec<_> = include_str!("../randomwords2.txt").split(' ').collect();

    // TODO: try increments up to 10000 to see when SeaHash overtakes FNV-1a
    c.bench_function(&format!("set {}", name), |b| {
        b.iter(|| {
            let mut map = MashHap::new(f);
            for word in &words {
                map.set(word, 5);
            }
        })
    });

    let mut preset_map = MashHap::new(f);
    for word in &words {
        preset_map.set(word, 5);
    }

    c.bench_function(&format!("get {}", name), |b| {
        b.iter(|| {
            for word in &words {
                preset_map.get(word);
            }
            for word in &words2 {
                preset_map.get(word);
            }
        })
    });

    c.bench_function(&format!("delete {}", name), |b| {
        b.iter(|| {
            for word in &words {
                preset_map.delete(word);
            }
            for word in &words2 {
                preset_map.delete(word);
            }
        })
    });

    c.bench_function(&format!("all three {}", name), |b| {
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
}

fn bench_fnv(c: &mut Criterion) {
    bench_suite(c, fnv_1a, "FNV-1a");
}

fn bench_seahash(c: &mut Criterion) {
    bench_suite(c, seahash, "SeaHash");
}

fn bench_murmurhash3(c: &mut Criterion) {
    bench_suite(c, murmurhash3, "MurmurHash 3");
}

criterion_group!(benches, bench_fnv, bench_seahash, bench_murmurhash3);
criterion_main!(benches);
