use drone_truck_solver::operator::{DeployDrone, ScoochLaunchAndLanding, SwapTrucks};
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

        let mut solver;

        let mut best_solution = problem.generate_initial_solution();
        let mut best_score = problem.calculate_score(&best_solution).unwrap();
        let mut best_weights = (0, 0, 0);
        
        for i in 0..=100 {
            for j in 0..=(100-i) {
                solver = Solver {
                    strategy: SimulatedAnnealing::new(SmallRng::seed_from_u64(RNG_SEED))
                        .add_operator(ScoochLaunchAndLanding, i as u32)
                        .add_operator(DeployDrone, j as u32)
                        .add_operator(SwapTrucks, 100 - i - j),
                };

                let (new_solution, new_score) = solver.solve(&problem);
                if new_score < best_score {
                    best_solution = new_solution;
                    best_score = new_score;
                    best_weights = (i, j, 100 - i - j);
                }
            }
        }

        println!("Best solution: {:?}", best_solution.to_submission_format());
        println!("Objective: {:.2}", best_score as f64 / 100.0);
        println!("Best weights: {:?}", best_weights);
    } else {
        panic!("pass the input file as argument");
    }
}
