use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use merge::ideal::{gen_ideal_n, hash_join_ideal, mem_random_read, mem_scan, mem_strided_4_scan, mem_strided_scan, rust_sort, sort_ideal, sort_merge_join_ideal};

const L1_PROBES_8 : [usize; 16] =
    [512, 1024, 1536, 2048, 2560, 3072, 3584, 4096,
     4608, 5120, 5632, 6144, 6656, 7168, 7680, 8192];

const L2_PROBES_8 : [usize; 24] =
    [4096, 8192, 12288, 16384, 20480, 24576, 28672, 32768,
     36864, 40960, 45056, 49152, 53248, 57344, 61440, 65536,
     69632, 73728, 77824, 81920, 86016, 90112, 94208, 98304];

const L3_PROBES_8 : [usize; 16] =
    [524288, 1048576, 1572864, 2097152, 2621440, 3145728, 3670016, 4194304,
     4718592, 5242880, 5767168, 6291456, 6815744, 7340032, 7864320, 8388608];

const L3_FINE_PROBES_8 : [usize; 48] =
    [131072, 262144, 393216, 524288, 655360, 786432, 917504, 1048576,
     1179648, 1310720, 1441792, 1572864, 1703936, 1835008, 1966080, 2097152,
     2228224, 2359296, 2490368, 2621440, 2752512, 2883584, 3014656, 3145728,
     3276800, 3407872, 3538944, 3670016, 3801088, 3932160, 4063232, 4194304,
     4325376, 4456448, 4587520, 4718592, 4849664, 4980736, 5111808, 5242880,
     5373952, 5505024, 5636096, 5767168, 5898240, 6029312, 6160384, 6291456];

const LONG_PROBES_8 : [usize; 8] =
    [2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456];

const ALL_PROBES : [usize; 112] =
    [512, 1024, 1536, 2048, 2560, 3072, 3584, 4096,
     4608, 5120, 5632, 6144, 6656, 7168, 7680, 8192,
     4096, 8192, 12288, 16384, 20480, 24576, 28672, 32768,
     36864, 40960, 45056, 49152, 53248, 57344, 61440, 65536,
     69632, 73728, 77824, 81920, 86016, 90112, 94208, 98304,
     524288, 1048576, 1572864, 2097152, 2621440, 3145728, 3670016, 4194304,
     4718592, 5242880, 5767168, 6291456, 6815744, 7340032, 7864320, 8388608,
     131072, 262144, 393216, 524288, 655360, 786432, 917504, 1048576,
     1179648, 1310720, 1441792, 1572864, 1703936, 1835008, 1966080, 2097152,
     2228224, 2359296, 2490368, 2621440, 2752512, 2883584, 3014656, 3145728,
     3276800, 3407872, 3538944, 3670016, 3801088, 3932160, 4063232, 4194304,
     4325376, 4456448, 4587520, 4718592, 4849664, 4980736, 5111808, 5242880,
     5373952, 5505024, 5636096, 5767168, 5898240, 6029312, 6160384, 6291456,
     2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456];

fn bench_mem_scan(c: &mut Criterion) {
    let sizes = LONG_PROBES_8;

    let mut group = c.benchmark_group("mem_scan");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || {
                    gen_ideal_n(n)
                },
                |input| {
                    let out = mem_scan(black_box(&input));
                    black_box(out);
                },
                BatchSize::LargeInput
            );
        });
    }
}

fn bench_mem_strided_scan(c: &mut Criterion) {
    let sizes = LONG_PROBES_8;

    let mut group = c.benchmark_group("mem_strided_scan");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || {
                    gen_ideal_n(n)
                },
                |input| {
                    let out = mem_strided_scan(black_box(&input));
                    black_box(out);
                },
                BatchSize::LargeInput
            );
        });
    }
}

fn bench_mem_strided_4_scan(c: &mut Criterion) {
    let sizes = LONG_PROBES_8;

    let mut group = c.benchmark_group("mem_strided_4_scan");
    for &n in &sizes {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || {
                    gen_ideal_n(n)
                },
                |input| {
                    let out = mem_strided_4_scan(black_box(&input));
                    black_box(out);
                },
                BatchSize::LargeInput
            );
        });
    }
}

fn bench_mem_random_read(c: &mut Criterion) {
    let sizes = ALL_PROBES;

    let mut group = c.benchmark_group("mem_random_read");
    for (i, &n) in (&sizes).iter().enumerate() {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(format!("{}: {}", i, n), &n, |b, &n| {
            b.iter_batched(
                || {
                    gen_ideal_n(n)
                },
                |input| {
                    let out = mem_random_read(black_box(&input));
                    black_box(out)
                },
                BatchSize::LargeInput
            );
        });
    }
}

fn bench_sort_ideal(c: &mut Criterion) {
    let sizes = ALL_PROBES;

    let mut group = c.benchmark_group("sort_ideal");
    for (i, &n) in (&sizes).iter().enumerate() {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(format!("{}: {}", i, n), &n, |b, &n| {
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

fn bench_rust_sort(c: &mut Criterion) {
    let sizes = ALL_PROBES;

    let mut group = c.benchmark_group("rust_sort");
    for (i, &n) in (&sizes).iter().enumerate() {
        group.throughput(Throughput::Bytes((n * size_of::<usize>()) as u64));

        group.bench_with_input(format!("{}: {}", i, n), &n, |b, &n| {
            b.iter_batched(
                || {
                    gen_ideal_n(n)
                },
                |input| {
                    let out = rust_sort(black_box(input));
                    black_box(out.len());
                },
                BatchSize::LargeInput
            );
        });
    }
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

criterion_group!(benches, bench_mem_scan, bench_mem_strided_scan, bench_mem_strided_4_scan, bench_mem_random_read,
    bench_sort_ideal, bench_rust_sort, bench_sort_merge_join_ideal, bench_hash_join_ideal);
criterion_main!(benches);