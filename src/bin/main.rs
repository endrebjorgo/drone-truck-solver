use drone_truck_solver::operator::{DeployDrone, OneInsert, ScoochLaunchAndLanding, SwapTrucks, TwoOpt};
use drone_truck_solver::problem::Problem;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::{TimedAdaptive};

use std::path::PathBuf;
use std::time::{Duration, Instant};
use rayon::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;

const RNG_SEED: u64 = 1337;
const INSTANCE_DIRECTORY: &'static str = "./assets";

fn main() {
    let deadline = Instant::now() + Duration::from_secs(10);

    let instance_dir = std::fs::read_dir(INSTANCE_DIRECTORY)
        .expect("unable to find directory");

    let instances: Vec<PathBuf> = instance_dir
        .map(|f| f.unwrap().path())
        .collect();

    if instances.len() > 8 {
        panic!("unable to process more than 8 instances in parallell");
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(instances.len())
        .build_global()
        .expect("unable to build thread pool");

    let problems: Vec<Problem> = instances
        .iter()
        .map(|instance| Problem::from_file(instance.to_str().unwrap()))
        .collect();

    let results: Vec<_> = problems
        .par_iter()
        .map(|problem| 
            Solver {
                strategy: TimedAdaptive::new(SmallRng::seed_from_u64(RNG_SEED), deadline)
                    .add_operator(DeployDrone, 1.0)
                    .add_operator(OneInsert, 1.0)
                    .add_operator(ScoochLaunchAndLanding, 1.0)
                    .add_operator(SwapTrucks, 1.0)
                    .add_operator(TwoOpt, 1.0)
            }.solve(&problem).1
        ).collect();

    let norm_results: Vec<f64> = results.iter().map(|&r| r as f64 / 100.0).collect();

    for i in 0..instances.len() {
        println!("{}: {}", instances[i].to_str().unwrap(), norm_results[i]);
    }
}
