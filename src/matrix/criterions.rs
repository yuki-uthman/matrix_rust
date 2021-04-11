use super::*;
use ::criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion::*;
use std::time::Duration;

pub fn bench_algorithm(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/algorithm");
    group.sample_size(10);
    // group.measurement_time(Duration::new(1000, 0));
    for i in [2000].iter() {
        group.bench_with_input(BenchmarkId::new("normal", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("rotate", i), i, |b, i| {
            b.iter(|| normal::calculate_with_rotation(black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_serial(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/serial");
    group.sample_size(10);
    // group.measurement_time(Duration::new(1000, 0));
    for i in [10, 20, 30, 40, 50, 60, 70, 80, 90, 100].iter() {
        group.bench_with_input(BenchmarkId::new("normal", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });

        // group.bench_with_input(BenchmarkId::new("with_rotation", i), i, |b, i| {
        //     b.iter(|| normal::calculate_with_rotation(black_box(*i)))
        // });

        // group.bench_with_input(BenchmarkId::new("mpsc", i), i, |b, i| {
        //     b.iter(|| with_mpsc2::calculate(1, black_box(*i)))
        // });

        group.bench_with_input(BenchmarkId::new("crossbeam", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(1, black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_speedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/speedup");
    group.sample_size(10);
    // group.measurement_time(Duration::new(1000, 0));
    for i in [700].iter() {
        group.bench_with_input(BenchmarkId::new("normal", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });
    }

    for i in [700].iter() {
        group.bench_with_input(BenchmarkId::new("cross ", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(2, black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_mpsc(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/mpsc");
    group.sample_size(10);
    group.measurement_time(Duration::new(100, 0));
    group.sampling_mode(SamplingMode::Flat);
    for i in [100, 200, 400, 800, 1600].iter() {
        group.bench_with_input(BenchmarkId::new("1", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("2", i), i, |b, i| {
            b.iter(|| with_mpsc2::calculate(2, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("3", i), i, |b, i| {
            b.iter(|| with_mpsc2::calculate(3, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("4", i), i, |b, i| {
            b.iter(|| with_mpsc2::calculate(4, black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/mutex");
    group.sample_size(10);
    group.measurement_time(Duration::new(100, 0));
    group.sampling_mode(SamplingMode::Flat);
    for i in [100, 200, 400, 800, 1600].iter() {
        group.bench_with_input(BenchmarkId::new("1", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("2", i), i, |b, i| {
            b.iter(|| with_mutex::calculate(2, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("3", i), i, |b, i| {
            b.iter(|| with_mutex::calculate(3, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("4", i), i, |b, i| {
            b.iter(|| with_mutex::calculate(4, black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_crossbeam(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/crossbeam");
    group.sample_size(10);
    group.measurement_time(Duration::new(100, 0));
    group.sampling_mode(SamplingMode::Flat);
    for i in [100, 200, 400, 800, 1600].iter() {
        group.bench_with_input(BenchmarkId::new("1", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("2", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(2, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("3", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(3, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("4", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(4, black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_library_2_thread(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/library");
    group.sample_size(10);
    // group.measurement_time(Duration::new(100, 0));
    for i in [2000].iter() {
        group.bench_with_input(BenchmarkId::new("base ", i), i, |b, i| {
            b.iter(|| normal::calculate(black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("mpsc ", i), i, |b, i| {
            b.iter(|| with_mpsc2::calculate(2, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("mutex", i), i, |b, i| {
            b.iter(|| with_mutex::calculate(2, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("cross", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(2, black_box(*i)))
        });
    }
    group.finish();
}

pub fn bench_library_3_thread(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion/library");
    group.sample_size(10);
    for i in [50, 100, 200, 300].iter() {
        group.bench_with_input(BenchmarkId::new("mpsc2", i), i, |b, i| {
            b.iter(|| with_mpsc2::calculate(3, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("mutex", i), i, |b, i| {
            b.iter(|| with_mutex::calculate(3, black_box(*i)))
        });

        group.bench_with_input(BenchmarkId::new("cross", i), i, |b, i| {
            b.iter(|| with_crossbeam::calculate(3, black_box(*i)))
        });
    }
    group.finish();
}

// pub fn bench_with_diff_thread(c: &mut Criterion) {
//     let mut group = c.benchmark_group("prob001/threads");
//     group.sample_size(100);
//     for i in [
//         1_000_000, 2_000_000, 3_000_000, 4_000_000, 5_000_000, 6_000_000, 7_000_000, 8_000_000,
//         9_000_000, 10_000_000,
//     ]
//     .iter()
//     {
//         // group.bench_with_input(BenchmarkId::new("1", i), i, |b, i| {
//         //     b.iter(|| normal::main(black_box(*i)))
//         // });

//         group.bench_with_input(BenchmarkId::new("mpsc", i), i, |b, i| {
//             b.iter(|| with_mpsc::main4(black_box(*i)))
//         });

//         group.bench_with_input(BenchmarkId::new("mutex", i), i, |b, i| {
//             b.iter(|| with_mutex::main4(black_box(*i)))
//         });

//         // group.bench_with_input(BenchmarkId::new("4", i), i, |b, i| {
//         //     b.iter(|| with_mpsc::main4(black_box(*i)))
//         // });
//     }
//     group.finish();
// }
