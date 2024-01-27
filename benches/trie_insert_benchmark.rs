// benches/trie_insert_benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion};
use you_autocomplete_me::Trie;

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("insert_words", |b| {
        b.iter(|| {
            let mut trie = Trie::default();
            trie.insert("apple".into());
            trie.insert("banana".into());
            trie.insert("orange".into());
            trie.insert("orange juice".into());
        });
    });

    c.bench_function("insert_gibberish", |b| {
        b.iter(|| {
            let mut trie = Trie::default();
            for _ in 0..100000 {
                trie.insert(uuid::Uuid::new_v4().to_string());
            }
        });
    });

    c.bench_function("insert_double_gibberish", |b| {
        b.iter(|| {
            let mut trie = Trie::default();
            for _ in 0..100 {
                trie.insert(format!("{}-{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4()));
            }
        });
    });

    c.bench_function("search", |b| {
        let mut trie = Trie::default();
        for _ in 0..100 {
            trie.insert(uuid::Uuid::new_v4().to_string());
        }
        b.iter(|| trie.search("sd", None));
    });

    c.bench_function("search_double_gibberish", |b| {
        let mut trie = Trie::default();
        for _ in 0..100 {
            trie.insert(format!("{}-{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4()));
        }
        b.iter(|| {
            trie.search("sd", None);
        });
    });

    c.bench_function("search_double_gibberish_many", |b| {
        let mut trie = Trie::default();
        for _ in 0..1000 {
            trie.insert(format!("{}-{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4()));
        }
        b.iter(|| {
            trie.search("sd", None);
        });
    });

    c.bench_function("search_long_double_gibberish", |b| {
        let mut trie = Trie::default();
        for _ in 0..1000 {
            trie.insert(format!("{}-{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4()));
        }
        b.iter(|| {
            trie.search("sdefg", None);
        });
    });

    c.bench_function("search_many_double_gibberish_long", |b| {
        let mut trie = Trie::default();
        for _ in 0..1000 {
            trie.insert(format!("{}-{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4()));
        }
        b.iter(|| {
            trie.search("sd6", None);
        });
    });
}

criterion_group!(benches, insert_benchmark);
criterion_main!(benches);
