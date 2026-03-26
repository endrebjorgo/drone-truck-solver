use crate::problem::Problem;
use crate::solution::Solution;
use crate::strategy::Strategy;

pub struct Solver<T: Strategy> {
    pub problem: Problem,
    pub strategy: T,
}

impl<T: Strategy> Solver<T> {
    pub fn solve(&mut self) -> Solution {
        return self.strategy.solve(&self.problem);
    }
}
