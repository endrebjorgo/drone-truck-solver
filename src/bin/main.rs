use drone_truck_solver::problem::Problem;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::RandomSearch;

use rand::SeedableRng;

fn main() {
    if let Some(input_path) = std::env::args().nth(1) {
        if std::fs::exists(&input_path).is_err() {
            panic!("waaaa");
        }

        let problem = Problem::from_file(&input_path);

        let rng = rand::rngs::SmallRng::seed_from_u64(69420);

        let mut solver = Solver {
            strategy: RandomSearch::new(rng),
        };

        let (solution, score) = solver.solve(&problem);

        println!("Best solution: {:?}", solution.to_submission_format());
        println!("Objective: {}", score);
    } else {
        panic!("pass the input file as argument");
    }

}
