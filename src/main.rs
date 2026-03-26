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
    let mut solver1 = Solver {
        problem: Problem::from_file("./assets/F_10.txt"),
        strategy: RandomSearch,
    };

    let solution1 = solver1.solve();
    println!("{:?}", solution1);

    let mut solver2 = Solver {
        problem: Problem::from_file("./assets/F_10.txt"),
        strategy: LocalSearch::new().add_operator(Swap),
    };

    let solution2 = solver2.solve();
    println!("{:?}", solution2);
}
