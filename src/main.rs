mod benchmark;
mod ising;
mod simulation;
mod utils;

use crate::simulation::run_simulation;
// use crate::benchmark::run_benchmarks;

fn main() {
    env_logger::init();
    let _ = run_simulation();
}
