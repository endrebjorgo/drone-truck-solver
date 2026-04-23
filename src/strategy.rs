use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use std::collections::HashSet;
use rand::{Rng, RngExt};
use rand::prelude::IndexedRandom;


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
    operators: Vec<Box<dyn Operator>>,
}

impl LocalSearch {
    pub fn new() -> Self {
        Self { operators: Vec::new() }
    }

    pub fn add_operator(mut self, operator: impl Operator + 'static) -> Self {
        self.operators.push(Box::new(operator));
        return self;
    }
}

impl Strategy for LocalSearch {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        let mut visited: HashSet<Solution> = HashSet::new();

        let mut best_solution = problem.generate_initial_solution(); 
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: initial solution unexpectedly unvalid");
        
        let mut neighborhood: Vec<Solution> = Vec::new();

        for _ in 0..10_000 {
            let mut did_improve = false;

            neighborhood.clear();
            for op in self.operators.iter(){
                neighborhood.append(&mut op.generate_neighborhood(&best_solution));
            }

            for neighbor in neighborhood.iter() {
                if visited.contains(neighbor) {
                    continue;
                } else {
                    assert_eq!(visited.insert(neighbor.clone()), true);
                }

                if let Some(score) = problem.calculate_score(neighbor) {
                    if score < best_score {
                        best_solution = neighbor.clone();
                        best_score = score;
                        did_improve = true;
                    }
                }
            }

            if !did_improve {
                return (best_solution, best_score);
            }
        }
        return (best_solution, best_score);
    }
}

pub struct SimulatedAnnealing {
    rng: Box<dyn Rng>,
    operators: Vec<Box<dyn Operator>>,
}

impl SimulatedAnnealing {
    pub fn new(rng: impl Rng + 'static) -> Self {
        Self { 
            rng: Box::new(rng), 
            operators: Vec::new() 
        }
    }

    pub fn add_operator(mut self, operator: impl Operator + 'static) -> Self {
        self.operators.push(Box::new(operator));
        return self;
    }
}

impl Strategy for SimulatedAnnealing {
    fn solve(&mut self, problem: &Problem) -> (Solution, u32) {
        // TODO: extract hard coded values as parameters
        // final_temp, 0.8 etc.

        let mut best_solution = problem.generate_initial_solution(); 
        let mut best_score = problem.calculate_score(&best_solution)
            .expect("ERROR: initial solution unexpectedly unvalid");

        let mut incumbent_solution = best_solution.clone();
        let mut incumbent_score = best_score;
        
        let mut neighborhood: Vec<Solution> = Vec::new();

        let mut deltas = Vec::new();

        for _ in 0..100 {
            neighborhood.clear();
            for op in self.operators.iter(){
                neighborhood.push(op.get_random_neighbor(&incumbent_solution, &mut self.rng));
            }
            
            let new_solution = neighborhood.choose(&mut self.rng)
                .expect("neighborhood was empty");

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
            neighborhood.clear();
            for op in self.operators.iter(){
                neighborhood.push(op.get_random_neighbor(&incumbent_solution, &mut self.rng));
            }   
            
            let new_solution = neighborhood.choose(&mut self.rng)
                .expect("neighborhood was empty");

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
