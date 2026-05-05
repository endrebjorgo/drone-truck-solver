use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use rand::Rng;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub struct LocalSearch {
    rng: Box<dyn Rng>,
    operators: Vec<Box<dyn Operator>>,
    weights: Vec<u32>,
}

impl LocalSearch {
    pub fn new(rng: impl Rng + 'static) -> Self {
        Self {
            rng: Box::new(rng),
            operators: Vec::new(),
            weights: Vec::new(),
        }
    }

    pub fn add_operator(mut self, operator: impl Operator + 'static, weight: u32) -> Self {
        self.operators.push(Box::new(operator));
        self.weights.push(weight);
        self
    }
}

impl super::Strategy for LocalSearch {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        let weighted_dist = WeightedIndex::new(&self.weights)
            .expect("no weights");

        let mut best_solution = problem.generate_with_heuristic();
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: heuristic solution unexpectedly invalid");

        for _ in 0..10_000 {
            let op_idx = weighted_dist.sample(&mut self.rng);
            let Some(new_solution) = self.operators[op_idx]
                .get_random_neighbor(&best_solution, &mut self.rng)
            else {
                continue;
            };

            if let Some(new_score) = problem.calculate_score(&new_solution) {
                if new_score < best_score {
                    best_solution = new_solution.clone();
                    best_score = new_score;
                }
            }
        }

        (best_solution, best_score)
    }
}
