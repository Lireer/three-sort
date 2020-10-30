use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use three_sort::*;

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");
    let samples: [[u8; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for (i, slice) in samples.iter().enumerate() {
        let mut test_slice = slice.clone();
        // group.bench_function(BenchmarkId::new("std_sort_u8", i), |b| {
        //     b.iter(|| std_sort(&mut slice))
        // });
        // group.bench_function(BenchmarkId::new("std_sort_unstable_u8", i), |b| {
        //     b.iter(|| std_sort_unstable(&mut slice))
        // });
        group.bench_function(BenchmarkId::new("sort_if_else", i), |b| {
            b.iter(|| sort_if_else(&mut test_slice))
        });
        let mut test_slice = slice.clone();
        group.bench_function(BenchmarkId::new("sort_match", i), |b| {
            b.iter(|| sort_if_else(&mut test_slice))
        });
    }
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
