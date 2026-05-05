use drone_truck_solver::operator::{DeployDrone, OneInsert, ScoochLaunchAndLanding, SwapTrucks, TwoOpt, ThreeOpt};
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
    let deadline = Instant::now() + Duration::from_secs(30);

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
        .map(|problem| {
            let rng = SmallRng::seed_from_u64(RNG_SEED);
            let alpha = 0.99996;
            let temperature = 5.0;
            let learning_rate = 0.5;
            Solver {
                strategy: TimedAdaptive::new(rng, deadline, alpha, temperature, learning_rate)
                    .add_operator(DeployDrone, 1.0)
                    .add_operator(OneInsert, 1.0)
                    .add_operator(ScoochLaunchAndLanding, 1.0)
                    .add_operator(SwapTrucks, 1.0)
                    .add_operator(TwoOpt, 1.0)
                    .add_operator(ThreeOpt, 1.0)
            }.solve(&problem)
        }).collect();


    let norm_results: Vec<f64> = results.iter().map(|r| r.1 as f64 / 100.0).collect();

    for i in 0..instances.len() {
        println!("{}: {}", instances[i].to_str().unwrap(), norm_results[i]);
        //println!("Best solution: {}", results[i].0.to_submission_format());
    }
}
