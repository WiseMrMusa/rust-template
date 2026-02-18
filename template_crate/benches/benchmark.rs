//! Benchmarks for template_crate

use criterion::{Criterion, black_box, criterion_group};
use template_crate::{add_small_integers, sub_small_integers};

/// Benchmarks for `add_small_integers` function
fn bench_add_small_integers(c: &mut Criterion) {
    c.bench_function("add_small_integers valid", |b| {
        b.iter(|| add_small_integers(black_box(50), black_box(30)))
    });

    c.bench_function("add_small_integers bound check", |b| {
        b.iter(|| add_small_integers(black_box(200), black_box(5)))
    });
}

/// Benchmarks for `sub_small_integers` function
fn bench_sub_small_integers(c: &mut Criterion) {
    c.bench_function("sub_small_integers valid", |b| {
        b.iter(|| sub_small_integers(black_box(50), black_box(30)))
    });

    c.bench_function("sub_small_integers bound check", |b| {
        b.iter(|| sub_small_integers(black_box(200), black_box(5)))
    });
}

/// Entry point for benchmarks
fn main() {
    criterion_group!(benches, bench_add_small_integers, bench_sub_small_integers);
    benches();
}
