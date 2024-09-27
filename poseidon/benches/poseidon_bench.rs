use criterion::{criterion_group, criterion_main, Criterion};
use mina_curves::pasta::Fp;
use mina_poseidon::constants::PlonkSpongeConstantsKimchi;
use mina_poseidon::pasta::fp_kimchi as SpongeParametersKimchi;
use mina_poseidon::poseidon::{ArithmeticSponge as Poseidon, Sponge};

pub fn bench_poseidon_kimchi(c: &mut Criterion) {
    let mut group = c.benchmark_group("Poseidon");
    group.sample_size(100);

    let mut poseidon =
        Poseidon::<Fp, PlonkSpongeConstantsKimchi>::new(SpongeParametersKimchi::static_params());

    // Chain of hashes, starting from a random value
    group.bench_function("poseidon_hash_kimchi", |b| {
        let mut hash: Fp = rand::random();

        b.iter(|| {
            poseidon.absorb(&vec![hash]);
            hash = poseidon.squeeze();
        })
    });

    group.finish();
}

criterion_group!(benches, bench_poseidon_kimchi);
criterion_main!(benches);
