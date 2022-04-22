//! ECS benchmarks based on <https://github.com/rust-gamedev/ecs_bench_suite>.

#![cfg(feature = "derive")]

mod add_remove;
mod frag_iter;
mod simple;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_simple_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_insert");
    group.bench_function("muds", |b| {
        let mut bench = simple::InsertBenchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_simple_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_iter");
    group.bench_function("muds", |b| {
        let mut bench = simple::IterBenchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_frag_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmented_iter");
    group.bench_function("muds", |b| {
        let mut bench = frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_add_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove");
    group.bench_function("muds", |b| {
        let mut bench = add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
}

criterion_group!(
    benchmarks,
    bench_simple_insert,
    bench_simple_iter,
    bench_frag_iter,
    bench_add_remove
);
criterion_main!(benchmarks);
