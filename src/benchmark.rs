use log::info;
use rand::SeedableRng;

use crate::ising::{Ising, IsingClassic, IsingOptimized};
use crate::utils::Stats;

fn benchmark(
    model: &mut impl Ising,
    rng: &mut impl rand::Rng,
    temp: f64,
    sweeps_per_run: usize,
    runs: usize,
) {
    let mut times = Vec::with_capacity(runs);

    for _ in 0..runs {
        let start = std::time::Instant::now();
        for _ in 0..sweeps_per_run {
            model.run_sweep(rng, temp);
        }
        times.push(start.elapsed().as_secs_f64());
    }

    let mean: f64 = times.mean();
    let std_dev = times.std_dev();

    info!(
        "mean time: {:.4} s (± {:.4} s) for {} sweeps",
        mean, std_dev, sweeps_per_run
    );
    info!(
        "fastest run: {:.4} s",
        times.iter().copied().fold(f64::INFINITY, f64::min)
    );
}

pub fn run_benchmarks() {
    let n = 2048;
    let test_temp = 2.269;
    let benchmark_sweeps = 100;
    let runs = 10;

    let mut rng_default = rand::rng();
    let mut rng_xoshiro = rand::rngs::Xoshiro256PlusPlus::from_seed([0; 32]);

    let mut model_classic = IsingClassic::new(n);
    let mut model_optimized = IsingOptimized::new(n);

    benchmark(
        &mut model_classic,
        &mut rng_default,
        test_temp,
        benchmark_sweeps,
        runs,
    );
    benchmark(
        &mut model_optimized,
        &mut rng_xoshiro,
        test_temp,
        benchmark_sweeps,
        runs,
    );
}
