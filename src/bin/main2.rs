use drone_truck_solver::operator::{DeployDrone, OneInsert, ScoochLaunchAndLanding, SwapTrucks, TwoOpt, ThreeOpt};
use drone_truck_solver::problem::Problem;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::{TimedAdaptive};

use std::time::{Duration, Instant};
use rand::SeedableRng;
use rand::rngs::SmallRng;

const RNG_SEED: u64 = 1337;

fn main() {
    let deadline = Instant::now() + Duration::from_secs(20);
    let problem = Problem::from_file("./assets/F_100.txt");

    let rng = SmallRng::seed_from_u64(RNG_SEED);
    let alpha = 0.999999;
    let temperature = 50000.0;
    let learning_rate = 0.7;

    let mut solver = Solver {
        strategy: TimedAdaptive::new(rng, deadline, alpha, temperature, learning_rate)
            .add_operator(DeployDrone, 1.0)
            .add_operator(OneInsert, 1.0)
            .add_operator(ScoochLaunchAndLanding, 1.0)
            .add_operator(SwapTrucks, 1.0)
            .add_operator(TwoOpt, 1.0)
            .add_operator(ThreeOpt, 1.0)
    };

    let result = solver.solve(&problem);

    println!("Best score: {:.2}", result.1 as f64 / 100.0);
    println!("Best sol: {}", result.0.to_submission_format());
    println!("Weights: {:?}", solver.strategy.weights);
}
