/*
A benchmark for execution time of batch verification through amortization.
Measures time with 1-1024 proofs, and we can expect it to be lower than verifying proofs individually.
Can be run with:
cargo criterion --bench amortization
 */
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use kimchi::bench::BenchmarkCtx;

const PROOFS: usize = 10;

pub fn amortization(c: &mut Criterion) {
    let mut group = c.benchmark_group("amortization");

    let ctx = BenchmarkCtx::new(1 << 16);
    let proof_and_public = ctx.create_proof();
    let proofs: Vec<_> = std::iter::repeat(proof_and_public)
        .take(1 << PROOFS)
        .collect();

    group.sample_size(10);
    for size in 0..=PROOFS {
        group.throughput(criterion::Throughput::Elements(1 << size));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("2^{size}")),
            &(),
            |b, _| {
                b.iter_batched(
                    || &proofs[0..(1 << size)],
                    |input| ctx.batch_verification(black_box(input)),
                    BatchSize::SmallInput,
                );
            },
        );
    }
}

criterion_group!(benches, amortization);
criterion_main!(benches);
