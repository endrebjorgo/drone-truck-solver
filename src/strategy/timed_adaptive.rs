use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use std::time::Instant;
use rand::{Rng, RngExt};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub struct TimedAdaptive {
    rng: Box<dyn Rng>,
    deadline: Instant,
    operators: Vec<Box<dyn Operator>>,
    weights: Vec<f64>,
}

impl TimedAdaptive {
    pub fn new(rng: impl Rng + 'static, deadline: Instant) -> Self {
        Self {
            rng: Box::new(rng),
            deadline,
            operators: Vec::new(),
            weights: Vec::new(),
        }
    }

    pub fn add_operator(mut self, operator: impl Operator + 'static, weight: f64) -> Self {
        self.operators.push(Box::new(operator));
        self.weights.push(weight);
        self
    }
}

impl super::Strategy for TimedAdaptive {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        let mut weighted_dist = WeightedIndex::new(&self.weights)
            .expect("no weights");

        let mut best_solution = problem.generate_with_heuristic();
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: heuristic solution unexpectedly invalid");

        let mut incumbent_solution = best_solution.clone();
        let mut incumbent_score = best_score;

        let mut iterations_since_improvement = 0;
        let learning_rate: f64 = 0.7;
        let mut op_scores: Vec<u32> = vec![0; self.operators.len()];
        let mut op_attempts: Vec<u32> = vec![0; self.operators.len()];

        let mut temperature = 10.0;
        let alpha = 0.99999;
        let mut iteration = 0;

        while Instant::now() < self.deadline {
            if iterations_since_improvement > 50 {
                let mut perturbed_solution = incumbent_solution.clone();
                let mut perturbed_score = incumbent_score;

                for i in 0..self.operators.len() {
                    if let Some(new_solution) = self.operators[i]
                        .get_random_neighbor(&perturbed_solution, &mut self.rng)
                    {
                        if let Some(new_score) = problem.calculate_score(&new_solution) {
                            perturbed_solution = new_solution;
                            perturbed_score = new_score;
                        }
                    }
                }
                incumbent_solution = perturbed_solution;
                incumbent_score = perturbed_score;
            }

            if iteration % 100 == 0 {
                for i in 0..self.weights.len() {
                    if op_attempts[i] != 0 {
                        self.weights[i] = self.weights[i] * (1.0 - learning_rate);
                        self.weights[i] += learning_rate * op_scores[i] as f64 / op_attempts[i] as f64;
                    }
                }

                let sum: f64 = self.weights.iter().sum();
                for i in 0..self.weights.len() {
                    self.weights[i] /= sum;
                }

                weighted_dist = WeightedIndex::new(&self.weights)
                    .expect("no weights");
            }

            let op_idx = weighted_dist.sample(&mut self.rng);
            op_attempts[op_idx] += 1;

            let Some(new_solution) = self.operators[op_idx]
                .get_random_neighbor(&incumbent_solution, &mut self.rng)
            else {
                continue;
            };

            if let Some(new_score) = problem.calculate_score(&new_solution) {
                if new_score < incumbent_score {
                    op_scores[op_idx] += 1;
                    incumbent_solution = new_solution.clone();
                    incumbent_score = new_score;
                    if incumbent_score < best_score {
                        op_scores[op_idx] += 1;
                        best_solution = incumbent_solution.clone();
                        best_score = incumbent_score;
                        iterations_since_improvement = 0;
                    }
                } else {
                    let p = 1.0 / ((new_score - incumbent_score) as f64 / temperature).exp();
                    if self.rng.random::<f64>() < p {
                        incumbent_solution = new_solution.clone();
                        incumbent_score = new_score;
                    }
                }
            }

            iterations_since_improvement += 1;
            iteration += 1;
            temperature *= alpha;
        }

        println!("No. iterations: {}, Final temp: {}", iteration, temperature);
        (best_solution, best_score)
    }
}
