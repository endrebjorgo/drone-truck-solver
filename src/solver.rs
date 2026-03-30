use crate::problem::Problem;
use crate::solution::Solution;
use crate::strategy::Strategy;

pub struct Solver<T: Strategy> {
    pub strategy: T,
}

impl<T: Strategy> Solver<T> {
    pub fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        return self.strategy.solve(problem);
    }
}
