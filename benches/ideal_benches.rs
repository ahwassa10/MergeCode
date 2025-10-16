use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use merge::ideal::{gen_ideal_n, hash_join_ideal, sort_ideal, sort_merge_join_ideal};

fn bench_gen_ideal(c: &mut Criterion) {
    let sizes = [100_000, 1_000_000];

    let mut group = c.benchmark_group("gen_ideal");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| gen_ideal_n(black_box(n)))
        });
    }
    group.finish();
}

fn bench_sort_ideal(c: &mut Criterion) {
    let sizes = [100_000usize, 1_000_000];

    let mut group = c.benchmark_group("sort_ideal");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || {
                    gen_ideal_n(n)
                },
                |input| {
                    let out = sort_ideal(&black_box(input));
                    black_box(out.len());
                },
                BatchSize::LargeInput
            );
        });

    }
    group.finish();
}

fn bench_sort_merge_join_ideal(c: &mut Criterion) {
    let sizes = [100_000usize, 1_000_000, 10_000_000];

    let mut group = c.benchmark_group("sort_merge_join_ideal");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let l = gen_ideal_n(n);
                    let r = gen_ideal_n(n);
                    (l, r)
                },
                |input| {
                    let out = sort_merge_join_ideal(&black_box(input.0), &black_box(input.1));
                    black_box(out.len());
                },
                BatchSize::LargeInput
            );
        });
    }
    group.finish();
}

fn bench_hash_join_ideal(c: &mut Criterion) {
    let sizes = [100_000usize, 1_000_000, 10_000_000];

    let mut group = c.benchmark_group("hash_join_ideal");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let l = gen_ideal_n(n);
                    let r = gen_ideal_n(n);
                    (l, r)
                },
                |input| {
                    let out = hash_join_ideal(&black_box(input.0), &black_box(input.1));
                    black_box(out.len());
                },
                BatchSize::LargeInput
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_gen_ideal, bench_sort_ideal, bench_sort_merge_join_ideal, bench_hash_join_ideal);
criterion_main!(benches);