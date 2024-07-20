use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use you_autocomplete_me::TrieNode;

fn compare_arc_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert");
    for i in [100, 1000].iter() {
        // group.bench_with_input(BenchmarkId::new("Arc", i), i, |b, i| {
        //     b.iter(|| {
        //         let mut trie = TrieNode::default();
        //         for _ in 0..*i {
        //             let b = uuid::Uuid::new_v4().to_string();
        //             trie.insert(b.as_str());
        //         }
        //     })
        // });

        group.bench_with_input(BenchmarkId::new("String", i), i, |b, i| {
            b.iter(|| {
                let mut trie = TrieNode::default();
                for _ in 0..*i {
                    let b = uuid::Uuid::new_v4().to_string();
                    trie.insert(b.as_str());
                }
            })
        });
    }
    group.finish();

    group = c.benchmark_group("Search");

    for i in [100, 1000].iter() {
        let mut trie = TrieNode::default();
        for _ in 0..*i {
            let b = uuid::Uuid::new_v4().to_string();
            trie.insert(b.as_str());
        }

        group.bench_function(BenchmarkId::new("String", i), |b| {
            b.iter(|| {
                trie.search("sd");
            })
        });
    }
    group.finish();
}

criterion_group!(benches, compare_arc_str);
criterion_main!(benches);
