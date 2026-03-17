mod problem;
mod solution;
mod solver;

use crate::problem::Problem;
use crate::solution::Solution;
use crate::solver::{Objective, Solver};

fn main() {
    let problem = Problem::from_file("./assets/F_10.txt");
    println!("{:?}", problem);

    let solver = Solver::minimize(problem);
    let solution = solver.generate_initial_solution();
    println!("{:?}", solution);
}
