use bls12_381::Scalar;
use rust_eth_kzg::{constants::BYTES_PER_BLOB, DASContext, ThreadCount, TrustedSetup};
use std::time::Instant;
use tracing_forest::util::LevelFilter;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const POLYNOMIAL_LEN: usize = 4096;

fn dummy_blob() -> [u8; BYTES_PER_BLOB] {
    let polynomial: Vec<_> = (0..POLYNOMIAL_LEN)
        .map(|i| -Scalar::from(i as u64))
        .collect();
    let blob: Vec<_> = polynomial
        .into_iter()
        .flat_map(|scalar| scalar.to_bytes_be())
        .collect();
    blob.try_into().unwrap()
}
fn main() {
    let trusted_setup = TrustedSetup::default();
    let blob = dummy_blob();

    let ctx = DASContext::with_threads(
        &trusted_setup,
        ThreadCount::SensibleDefault,
        bls12_381::fixed_base_msm::UsePrecomp::Yes { width: 8 },
    );

    println!("Warming up for 3 seconds...");

    let start = Instant::now();
    while Instant::now().duration_since(start).as_secs() < 3 {
        ctx.compute_cells_and_kzg_proofs(&blob).unwrap();
    }

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    ctx.compute_cells_and_kzg_proofs(&blob).unwrap();
}
