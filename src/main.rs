mod operator;
mod problem;
mod solution;
mod solver;
mod strategy;
mod time_matrix;

use crate::operator::{Swap};
use crate::problem::Problem;
use crate::solver::Solver;
use crate::strategy::{LocalSearch, RandomSearch};

fn main() {
    let problem = Problem::from_file("./assets/F_10.txt");

    let mut solver1 = Solver {
        strategy: RandomSearch,
    };

    let solution1 = solver1.solve(&problem);

    let mut solver2 = Solver {
        strategy: LocalSearch::new().add_operator(Swap),
    };

    let solution2 = solver2.solve(&problem);

    println!("{:?}", solution1.to_submission_format());
    println!("{:?}", solution2.to_submission_format());
}
