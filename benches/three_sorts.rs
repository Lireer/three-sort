use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use three_sort::*;

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");
    let samples: [[u64; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for (i, slice) in samples.iter().enumerate() {
        group.bench_function(BenchmarkId::new("std_sort_unstable", i), |b| {
            b.iter_batched_ref(
                || slice.clone(),
                |mut slice| std_sort_unstable(&mut slice),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("bubble_sort", i), |b| {
            b.iter_batched_ref(
                || slice.clone(),
                |mut slice| bubble_sort(&mut slice),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("if_else_sort", i), |b| {
            b.iter_batched_ref(
                || slice.clone(),
                |mut slice| if_else_sort(&mut slice),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("match_sort", i), |b| {
            b.iter_batched_ref(
                || slice.clone(),
                |mut slice| match_sort(&mut slice),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("min_max_sort", i), |b| {
            b.iter_batched_ref(
                || slice.clone(),
                |mut slice| min_max_sort_u64(&mut slice),
                BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
