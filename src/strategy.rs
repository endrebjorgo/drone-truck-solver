use crate::operator::Operator;
use crate::problem::Problem;
use crate::solution::Solution;

use std::collections::HashSet;
use rand::Rng;

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
        
        let mut neighborhood: Vec<Solution>;

        for _ in 0..10_000 {
            let mut did_improve = false;

            neighborhood = Vec::new();
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

