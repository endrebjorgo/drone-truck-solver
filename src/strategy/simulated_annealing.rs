use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use rand::{Rng, RngExt};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub struct SimulatedAnnealing {
    rng: Box<dyn Rng>,
    operators: Vec<Box<dyn Operator>>,
    weights: Vec<u32>,
}

impl SimulatedAnnealing {
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

impl super::Strategy for SimulatedAnnealing {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        let weighted_dist = WeightedIndex::new(&self.weights)
            .expect("no weights");

        let mut best_solution = problem.generate_with_heuristic();
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: heuristic solution unexpectedly invalid");

        let mut incumbent_solution = best_solution.clone();
        let mut incumbent_score = best_score;

        let mut deltas = Vec::new();

        for _ in 0..100 {
            let op_idx = weighted_dist.sample(&mut self.rng);
            let Some(new_solution) = self.operators[op_idx]
                .get_random_neighbor(&incumbent_solution, &mut self.rng)
            else {
                continue;
            };

            if let Some(new_score) = problem.calculate_score(&new_solution) {
                if new_score < incumbent_score {
                    incumbent_solution = new_solution.clone();
                    incumbent_score = new_score;
                    if incumbent_score < best_score {
                        best_solution = incumbent_solution.clone();
                        best_score = incumbent_score;
                    }
                } else {
                    deltas.push(new_score - incumbent_score);
                    if self.rng.random::<f64>() < 0.8f64 {
                        incumbent_solution = new_solution.clone();
                        incumbent_score = new_score;
                    }
                }
            }
        }

        let delta_avg: f64 = deltas.iter().sum::<u32>() as f64 / deltas.len() as f64;
        let mut temperature = -delta_avg / 0.8f64.ln();
        let final_temperature = 0.1;
        let alpha = (final_temperature / temperature).powf(1.0 / 9900.0);

        for _ in 0..9900 {
            let op_idx = weighted_dist.sample(&mut self.rng);
            let Some(new_solution) = self.operators[op_idx]
                .get_random_neighbor(&incumbent_solution, &mut self.rng)
            else {
                continue;
            };

            if let Some(new_score) = problem.calculate_score(&new_solution) {
                if new_score < incumbent_score {
                    incumbent_solution = new_solution.clone();
                    incumbent_score = new_score;
                    if incumbent_score < best_score {
                        best_solution = incumbent_solution.clone();
                        best_score = incumbent_score;
                    }
                } else {
                    let p = 1.0 / ((new_score - incumbent_score) as f64 / temperature).exp();
                    if self.rng.random::<f64>() < p {
                        incumbent_solution = new_solution.clone();
                        incumbent_score = new_score;
                    }
                }
            }

            temperature *= alpha;
        }

        (best_solution, best_score)
    }
}
