extern crate criterion;
use ::matrix::*;
use criterion::{criterion_group, criterion_main};

criterion_group!(
    bench,
    // matrix::criterions::bench_serial,
    matrix::criterions::bench_speedup,
    // matrix::criterions::bench_algorithm,
    // matrix::criterions::bench_mpsc,
    // matrix::criterions::bench_mutex,
    // matrix::criterions::bench_crossbeam,
    // matrix::criterions::bench_library_2_thread,
);
criterion_main!(bench);
