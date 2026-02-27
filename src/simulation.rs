use log::info;
use rand::SeedableRng;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ising::{Ising, IsingOptimized};
use crate::utils::Stats;

pub fn run_simulation() -> std::io::Result<()> {
    let n = 1024;
    let thermalization_sweeps = 1000;
    let measurement_sweeps = 500;
    let t_max: f64 = 4.0;
    let t_critical: f64 = 2.269;

    let mut rng = rand::rngs::Xoshiro256PlusPlus::seed_from_u64(0);

    let file = File::create("magnetization_data.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "temp,mag,std_dev")?;

    info!(
        "starting simulation with n={}, thermalization_sweeps={}, measurement_sweeps={}",
        n, thermalization_sweeps, measurement_sweeps
    );
    let mut t: f64 = 1.0;
    let start = std::time::Instant::now();
    while t <= t_max {
        // calcualting temperature step based on distance from critical point
        let dt = if (t - t_critical).abs() < 0.2 {
            0.01 // dense 
        } else if (t - t_critical).abs() < 0.5 {
            0.04 // middle step size around critical point
        } else {
            0.1 // big step size far from critical point
        };
        let mut model = IsingOptimized::new(n);

        let is_near_critical = (t - t_critical).abs() < 0.1;
        let current_measurements = if is_near_critical {
            measurement_sweeps * 2
        } else {
            measurement_sweeps
        };

        // thermalisation phase
        for _ in 0..thermalization_sweeps {
            model.run_sweep(&mut rng, t);
        }

        let mut measurements = Vec::with_capacity(current_measurements);

        for _ in 0..current_measurements {
            model.run_sweep(&mut rng, t);
            measurements.push(model.calculate_magnetization().abs());
        }
        let avg_mag = measurements.mean();
        let std_dev = measurements.std_dev();

        info!("T={:.2}: |M| = {:.4} ± {:.4}", t, avg_mag, std_dev);
        writeln!(writer, "{:.4},{:.5},{:.5}", t, avg_mag, std_dev)?;

        t += dt;
    }
    let finish = start.elapsed().as_secs_f64();

    writer.flush()?;
    info!("simulation finished in {:.2} seconds", finish);
    Ok(())
}
