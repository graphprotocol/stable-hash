use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, thread_rng, RngCore as _, SeedableRng as _};
use stable_hash::fld_mixer::{FldMix, FldMixScalar};
#[cfg(feature = "simd")]
use stable_hash::fld_mixer_simd::FldMixSimd;
use stable_hash::utils::StableHasherWrapper;
use stable_hash::*;
use twox_hash::XxHash64;

fn bench_mixer<M: FldMix + Default>(inputs: &[u64]) -> u64 {
    let mut hasher = StableHasherWrapper::<XxHash64, M, SequenceNumberInt<u64>>::default();
    for input in inputs {
        hasher.write(SequenceNumberInt::root(), &input.to_le_bytes());
    }
    hasher.finish()
}

fn bench_mixers(c: &mut Criterion) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let mut inputs = black_box(Vec::new());
    let n = 64;
    for _ in 0..n {
        inputs.push(rng.next_u64());
    }

    let mut group = c.benchmark_group("fld_mixer");
    group.bench_function(&format!("scalar {}", n), |b| {
        b.iter(|| bench_mixer::<FldMixScalar>(&inputs))
    });
    #[cfg(feature = "simd")]
    group.bench_function(&format!("simd {}", n), |b| {
        b.iter(|| bench_mixer::<FldMixSimd>(&inputs))
    });
    group.finish();
}

criterion_group!(benches, bench_mixers);
criterion_main!(benches);
