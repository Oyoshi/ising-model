mod benchmark;
mod ising;
mod simulation;
mod utils;

use crate::benchmark::run_benchmarks;
use crate::simulation::run_simulation;

fn main() {
    env_logger::init();
    // run_benchmarks();
    let _ = run_simulation();
}
