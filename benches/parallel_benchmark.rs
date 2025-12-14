use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rayon::prelude::*;
use xor_linked_list::XorLinkedList;

fn create_list(size: usize) -> XorLinkedList<i32> {
    let mut list = XorLinkedList::new();
    for i in 0..size as i32 {
        list.push_back(i);
    }
    list
}

fn bench_parallel_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        let list = create_list(*size);

        group.bench_with_input(BenchmarkId::new("parallel", size), size, |b, _| {
            b.iter(|| {
                let sum: i32 = list.par_iter().sum();
                black_box(sum)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter(|| {
                let sum: i32 = list.iter().sum();
                black_box(sum)
            });
        });
    }

    group.finish();
}

fn bench_parallel_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_collect");

    for size in [100, 1_000, 10_000].iter() {
        let list = create_list(*size);

        group.bench_with_input(BenchmarkId::new("parallel", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<i32> = list.par_iter().map(|&x| x * 2).collect();
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<i32> = list.iter().map(|&x| x * 2).collect();
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_parallel_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter");

    for size in [100, 1_000, 10_000].iter() {
        let list = create_list(*size);

        group.bench_with_input(BenchmarkId::new("parallel", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<&i32> = list.par_iter().filter(|&&x| x % 2 == 0).collect();
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter(|| {
                let result: Vec<&i32> = list.iter().filter(|&&x| x % 2 == 0).collect();
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_parallel_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("find");

    for size in [1_000, 10_000, 100_000].iter() {
        let list = create_list(*size);
        let target = (*size as i32) / 2; // Find middle element

        group.bench_with_input(BenchmarkId::new("parallel", size), size, |b, _| {
            b.iter(|| {
                let result = list.par_iter().find_any(|&&x| x == target);
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter(|| {
                let result = list.iter().find(|&&x| x == target);
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_parallel_fold_reduce(c: &mut Criterion) {
    let mut group = c.benchmark_group("fold_reduce");

    for size in [100, 1_000, 10_000].iter() {
        let list = create_list(*size);

        group.bench_with_input(BenchmarkId::new("parallel", size), size, |b, _| {
            b.iter(|| {
                let result = list
                    .par_iter()
                    .fold(|| 0i64, |acc, &x| acc + x as i64)
                    .reduce(|| 0i64, |a, b| a + b);
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, _| {
            b.iter(|| {
                let result: i64 = list.iter().map(|&x| x as i64).sum();
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_parallel_any_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("any_all");

    for size in [1_000, 10_000, 100_000].iter() {
        let list = create_list(*size);

        group.bench_with_input(BenchmarkId::new("parallel_any", size), size, |b, _| {
            b.iter(|| {
                let result = list.par_iter().any(|&x| x == -1); // Not found
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential_any", size), size, |b, _| {
            b.iter(|| {
                let result = list.iter().any(|&x| x == -1); // Not found
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("parallel_all", size), size, |b, _| {
            b.iter(|| {
                let result = list.par_iter().all(|&x| x >= 0);
                black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential_all", size), size, |b, _| {
            b.iter(|| {
                let result = list.iter().all(|&x| x >= 0);
                black_box(result)
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parallel_sum,
    bench_parallel_map,
    bench_parallel_filter,
    bench_parallel_find,
    bench_parallel_fold_reduce,
    bench_parallel_any_all
);
criterion_main!(benches);
