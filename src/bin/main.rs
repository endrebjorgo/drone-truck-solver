use drone_truck_solver::operator::{ScoochLaunchAndLanding, SwapTrucks, OneInsert, DeployDrone};
use drone_truck_solver::problem::Problem;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::{GeneralAdaptive};

use rand::SeedableRng;
use rand::rngs::SmallRng;

const RNG_SEED: u64 = 1337;

fn main() {
    if let Some(input_path) = std::env::args().nth(1) {
        if std::fs::exists(&input_path).is_err() {
            panic!("waaaa");
        }

        let problem = Problem::from_file(&input_path);

        let mut solver = Solver {
            strategy: GeneralAdaptive::new(SmallRng::seed_from_u64(RNG_SEED))
                .add_operator(DeployDrone, 1.0)
                .add_operator(OneInsert, 1.0)
                .add_operator(ScoochLaunchAndLanding, 1.0)
                .add_operator(SwapTrucks, 1.0)
        };

        let (solution, score) = solver.solve(&problem);

        println!("Best solution: {:?}", solution.to_submission_format());
        println!("Objective: {:.2}", score as f64 / 100.0);
    } else {
        panic!("pass the input file as argument");
    }
}
