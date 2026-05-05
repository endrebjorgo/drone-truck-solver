use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use std::time::{Duration, Instant};
use rand::{Rng, RngExt};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub struct TimedAdaptive {
    pub rng: Box<dyn Rng>,
    pub deadline: Instant,
    pub alpha: f64,
    pub temperature: f64,
    pub learning_rate: f64,
    pub operators: Vec<Box<dyn Operator>>,
    pub weights: Vec<f64>,
}

impl TimedAdaptive {
    pub fn new(
        rng: impl Rng + 'static, 
        deadline: Instant, 
        alpha: f64, 
        temperature: f64, 
        learning_rate: f64
    ) -> Self {
        Self {
            rng: Box::new(rng),
            deadline,
            alpha,
            temperature,
            learning_rate,
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

        let found_incumbent_score = 2.0;
        let found_best_score = 4.0;

        let mut best_solution = problem.generate_with_heuristic();
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("error: heuristic solution unexpectedly invalid");

        let mut incumbent_solution = best_solution.clone();
        let mut incumbent_score = best_score;

        let mut op_scores: Vec<f64> = vec![0.0; self.operators.len()];
        let mut op_attempts: Vec<u32> = vec![0; self.operators.len()];

        let mut iteration = 0;
        let mut iterations_since_improvement = 0;

        let mut next_print = Instant::now() + Duration::from_secs(60);

        while Instant::now() < self.deadline {
            if Instant::now() > next_print {
                next_print = Instant::now() + Duration::from_secs(60);
                println!("Best objective: {}", best_score);
            }

            iterations_since_improvement += 1;
            iteration += 1;

            // update weights
            if iteration % 100 == 0 {
                for i in 0..self.weights.len() {
                    if op_attempts[i] != 0 {
                        self.weights[i] = self.weights[i] * (1.0 - self.learning_rate);
                        self.weights[i] += self.learning_rate * op_scores[i] / op_attempts[i] as f64;
                    }
                }

                let sum: f64 = self.weights.iter().sum();
                for i in 0..self.weights.len() {
                    self.weights[i] /= sum;
                }

                weighted_dist = WeightedIndex::new(&self.weights)
                    .expect("no weights");
            }

            // perturb incumbent
            if iterations_since_improvement > 100 { 
                if let Some(solution) = problem.remove_moot_flights(&incumbent_solution) {
                    if let Some(score) = problem.calculate_score(&solution) {
                        incumbent_solution = solution;
                        incumbent_score = score;
                    }
                }

                for _ in 0..5 {
                    let op_idx = self.rng.random_range(0..self.operators.len());
                    op_attempts[op_idx] += 1;

                    if let Some(new_solution) = self.operators[op_idx]
                        .get_random_neighbor(&incumbent_solution, &mut self.rng)
                    {
                        if let Some(new_score) = problem.calculate_score(&new_solution) {
                            incumbent_solution = new_solution;
                            incumbent_score = new_score;
                            op_scores[op_idx] += found_incumbent_score;
                        }
                    }
                }
            }

            // generate all solutions instead of randomizing
            if iterations_since_improvement > 100000 {
                let mut improved = false;
                for (op_idx, op) in self.operators.iter().enumerate() {
                    op_attempts[op_idx] += 1;

                    let neighborhood = op.generate_neighborhood(&incumbent_solution);

                    for neighbor in neighborhood {
                        if let Some(score) = problem.calculate_score(&neighbor) {
                            if score < incumbent_score {
                                incumbent_solution = neighbor.clone();
                                incumbent_score = score;
                                op_scores[op_idx] += found_incumbent_score;
                            }
                        } 
                    }
                    if incumbent_score < best_score {
                        best_solution = incumbent_solution.clone();
                        best_score = incumbent_score;
                        op_scores[op_idx] += found_best_score;
                        improved = true;

                        iterations_since_improvement = 0;
                        break;
                    } 
                }
                if !improved {
                    // no better solution in this neighborhood...
                    let new_start = problem.generate_with_random_heuristic(&mut self.rng);
                    if let Some(new_start_score) = problem.calculate_score(&new_start) {
                        incumbent_solution = new_start;
                        incumbent_score = new_start_score;

                        self.temperature /= self.alpha.powi(iterations_since_improvement);
                        iterations_since_improvement = 0;
                    }
                }
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
                    incumbent_solution = new_solution.clone();
                    incumbent_score = new_score;
                    op_scores[op_idx] += found_incumbent_score;

                    if incumbent_score < best_score {
                        best_solution = incumbent_solution.clone();
                        best_score = incumbent_score;

                        op_scores[op_idx] += found_best_score;
                        iterations_since_improvement = 0;
                    }
                } else {
                    let p = 1.0 / ((new_score - incumbent_score) as f64 / self.temperature).exp();
                    if self.rng.random::<f64>() < p {
                        incumbent_solution = new_solution.clone();
                        incumbent_score = new_score;
                    }
                }
            }

            self.temperature *= self.alpha;
        }

        return (best_solution, best_score);
    }
}
