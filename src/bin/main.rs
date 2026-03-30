use drone_truck_solver::operator::{Swap};
use drone_truck_solver::problem::Problem;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::{LocalSearch, RandomSearch};

use rand::SeedableRng;

fn main() {
    let problem = Problem::from_file("./assets/F_10.txt");

    let rng = rand::rngs::SmallRng::seed_from_u64(69420);

    let mut solver1 = Solver {
        strategy: RandomSearch::new(rng),
    };

    let (solution1, score1) = solver1.solve(&problem);

    let mut solver2 = Solver {
        strategy: LocalSearch::new().add_operator(Swap),
    };

    let (solution2, score2) = solver2.solve(&problem);

    println!("{:?}", solution1.to_submission_format());
    println!("{:?}", solution2.to_submission_format());
}
