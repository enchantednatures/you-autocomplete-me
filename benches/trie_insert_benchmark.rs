// benches/trie_insert_benchmark.rs

use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use you_autocomplete_me::AutocompletionEngine;

fn compare_arc_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert");
    for i in [100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("Arc", i), i, |b, i| {
            b.iter(|| {
                let mut trie = AutocompletionEngine::default();
                for _ in 0..*i {
                    let b = uuid::Uuid::new_v4().to_string();
                    let bytes = b.as_bytes();
                    let b2: Arc<str> = b.clone().into();
                    trie.insert(bytes, b2);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("String", i), i, |b, i| {
            b.iter(|| {
                let mut trie = AutocompletionEngine::default();
                for _ in 0..*i {
                    let b = uuid::Uuid::new_v4().to_string();
                    let bytes = b.as_bytes();
                    let b2: String = b.clone();
                    trie.insert(bytes, b2);
                }
            })
        });
    }
    group.finish();

    group = c.benchmark_group("Search");

    for i in [100, 1000].iter() {
        let mut trie = AutocompletionEngine::default();
        for _ in 0..*i {
            let b = uuid::Uuid::new_v4().to_string();
            let bytes = b.as_bytes();
            let b2: Arc<str> = b.clone().into();
            trie.insert(bytes, b2);
        }
        group.bench_function(BenchmarkId::new("Arc", i), |b| {
            b.iter(|| {
                trie.full_text_search("sd".as_bytes(), None);
            })
        });

        let mut trie = AutocompletionEngine::default();
        for _ in 0..*i {
            let b = uuid::Uuid::new_v4().to_string();
            let bytes = b.as_bytes();
            let b2: Arc<str> = b.clone().into();
            trie.insert(bytes, b2);
        }

        group.bench_function(BenchmarkId::new("String", i), |b| {
            b.iter(|| {
                trie.full_text_search("sd".as_bytes(), None);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, compare_arc_str);
criterion_main!(benches);
