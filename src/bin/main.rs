use drone_truck_solver::operator::{OneInsert};
use drone_truck_solver::problem::Problem;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::{LocalSearch, SimulatedAnnealing};

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
            strategy: SimulatedAnnealing::new(SmallRng::seed_from_u64(RNG_SEED))
                .add_operator(OneInsert, 1),
        };

        let (solution, score) = solver.solve(&problem);

        println!("Best solution: {:?}", solution.to_submission_format());
        println!("Objective: {}", score);
    } else {
        panic!("pass the input file as argument");
    }
}
