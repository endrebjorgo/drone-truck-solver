use crate::problem::Problem;
use crate::solution::Solution;

use rand::Rng;

pub struct RandomSearch {
    pub rng: Box<dyn Rng>,
}

impl RandomSearch {
    pub fn new(rng: impl Rng + 'static) -> Self {
        Self { rng: Box::new(rng) }
    }
}

impl super::Strategy for RandomSearch {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        let mut best_solution = problem.generate_initial_solution();
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: initial solution unexpectedly invalid");

        let mut current_solution;

        for _ in 0..10_000 {
            current_solution = problem.generate_random_solution(&mut self.rng);
            if let Some(score) = problem.calculate_score(&current_solution) {
                if score < best_score {
                    best_solution = current_solution;
                    best_score = score;
                    continue;
                }
            }
        }
        (best_solution, best_score)
    }
}
