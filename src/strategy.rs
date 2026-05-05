use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use std::time::Instant;
use rand::{Rng, RngExt};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;


pub trait Strategy {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32);
}

pub struct RandomSearch {
    rng: Box<dyn Rng>,
}

impl RandomSearch {
    pub fn new(rng: impl Rng + 'static) -> Self {
        Self { rng: Box::new(rng) }
    }
}

impl Strategy for RandomSearch {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        let mut best_solution = problem.generate_initial_solution(); 
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: initial solution unexpectedly unvalid");
        
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
        return (best_solution, best_score);
    }
}

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
        return self;
    }
}

impl Strategy for LocalSearch {
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

        return (best_solution, best_score);
    }
}

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
        return self;
    }
}

impl Strategy for SimulatedAnnealing {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        // TODO: extract hard coded values as parameters
        // final_temp, 0.8 etc.

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

        return (best_solution, best_score);
    }
}

pub struct GeneralAdaptive {
    rng: Box<dyn Rng>,
    operators: Vec<Box<dyn Operator>>,
    weights: Vec<f64>,
}

impl GeneralAdaptive {
    pub fn new(rng: impl Rng + 'static) -> Self {
        Self { 
            rng: Box::new(rng), 
            operators: Vec::new(),
            weights: Vec::new(),
        }
    }

    pub fn add_operator(mut self, operator: impl Operator + 'static, weight: f64) -> Self {
        self.operators.push(Box::new(operator));
        self.weights.push(weight);
        return self;
    }
}

impl Strategy for GeneralAdaptive {
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

        let max_iterations = 10_000;

        let mut temperature: f64 = 5.0;
        let final_temperature: f64 = 0.1;
        let alpha = (final_temperature / temperature).powf(1.0 / (max_iterations as f64));

        for iteration in 0..max_iterations {
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
                iterations_since_improvement = 0;
            }

            if iterations_since_improvement > 100 {
                if let Some(solution) = problem.remove_moot_nodes(&incumbent_solution) {
                    incumbent_solution = solution;
                    incumbent_score = problem.calculate_score(&incumbent_solution)
                        .expect("remove moot nodes destroyed something..");
                }
                iterations_since_improvement = 0;
            }
            
            if iterations_since_improvement > 10_000 {
                break;
            }

            if iteration % 100 == 0 {
                // Update weights based on scores
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
                    let p = 1.0 / (new_score.abs_diff(incumbent_score) as f64 / temperature).exp();

                    if self.rng.random::<f64>() < p {
                        incumbent_solution = new_solution.clone();
                        incumbent_score = new_score;
                    }
                }
            }

            iterations_since_improvement += 1;
            temperature *= alpha;
        }

        return (best_solution, best_score);
    }
}

pub struct FinalStrategy {
    rng: Box<dyn Rng>,
    deadline: Instant,
    operators: Vec<Box<dyn Operator>>,
    weights: Vec<f64>,
}

impl FinalStrategy {
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
        return self;
    }
}

impl Strategy for FinalStrategy {
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

        let mut temperature = 5.0;
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
                iterations_since_improvement = 0;
            }

            if iterations_since_improvement > 100 {
                if let Some(solution) = problem.remove_moot_nodes(&incumbent_solution) {
                    incumbent_solution = solution;
                    incumbent_score = problem.calculate_score(&incumbent_solution)
                        .expect("remove moot nodes destroyed something..");
                }
                iterations_since_improvement = 0;
            }
            
            if iteration % 100 == 0 {
                // Update weights based on scores
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
                    let p = 1.0 / (new_score.abs_diff(incumbent_score) as f64 / temperature).exp();

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

        return (best_solution, best_score);
    }
}
