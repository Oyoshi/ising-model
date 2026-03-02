use log::info;
use rand::SeedableRng;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use crate::ising::{Ising, IsingOptimized};
use crate::utils::Stats;

struct SimResult {
    temp: f64,
    mag: f64,
    std_dev: f64,
}

pub fn run_simulation() -> std::io::Result<()> {
    let n = 1024;
    let thermalization_sweeps = 1000;
    let measurement_sweeps = 500;
    let t_max: f64 = 4.0;
    let t_critical: f64 = 2.269;

    let file = File::create("magnetization_data.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "temp,mag,std_dev")?;

    info!(
        "starting simulation with n={}, thermalization_sweeps={}, measurement_sweeps={}",
        n, thermalization_sweeps, measurement_sweeps
    );

    let start = Instant::now();

    let mut temps = Vec::new();
    let mut current_t = 1.0;
    while current_t <= t_max {
        temps.push(current_t);
        let dt = if (current_t - t_critical).abs() < 0.2 {
            0.01
        } else if (current_t - t_critical).abs() < 0.5 {
            0.04
        } else {
            0.1
        };
        current_t += dt;
    }

    let mut results: Vec<SimResult> = temps
        .into_par_iter()
        .map(|t| {
            let mut thread_rng = rand::rngs::Xoshiro256PlusPlus::seed_from_u64((t * 1000.0) as u64);

            let mut model = IsingOptimized::new(n);

            let is_near_critical = (t - t_critical).abs() < 0.1;
            let current_measurements = if is_near_critical {
                measurement_sweeps * 2
            } else {
                measurement_sweeps
            };

            for _ in 0..thermalization_sweeps {
                model.run_sweep(&mut thread_rng, t);
            }

            let mut measurements: Vec<f64> = Vec::with_capacity(current_measurements);

            for _ in 0..current_measurements {
                model.run_sweep(&mut thread_rng, t);
                measurements.push(model.calculate_magnetization().abs());
            }

            let avg_mag = measurements.mean();
            let std_dev = measurements.std_dev();

            info!(
                "Finished T={:.3} on thread {:?}",
                t,
                std::thread::current().id()
            );

            SimResult {
                temp: t,
                mag: avg_mag,
                std_dev,
            }
        })
        .collect();

    results.sort_by(|a, b| a.temp.partial_cmp(&b.temp).unwrap());

    let file = File::create("magnetization_data.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "temp,mag,std_dev")?;

    for res in results {
        writeln!(writer, "{:.4},{:.5},{:.5}", res.temp, res.mag, res.std_dev)?;
    }

    let finish = start.elapsed().as_secs_f64();

    writer.flush()?;
    info!("simulation finished in {:.2} seconds", finish);
    Ok(())
}
