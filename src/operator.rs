use crate::solution::Solution;

pub trait Operator {
    fn generate_neighbors(&self, solution: &Solution) -> Vec<Solution>;
}

pub struct Swap;

impl Operator for Swap {
    fn generate_neighbors(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighbors: Vec<Solution> = Vec::new();

        for i in 1..(solution.truck_path.len() - 1) {
            for j in (i + 1)..(solution.truck_path.len() - 1) {
                if i == j { continue; }
                
                let mut new_neighbor = solution.clone();
                new_neighbor.truck_path.swap(i, j);
                neighbors.push(new_neighbor);
            }
        }
        return neighbors;
    }
}

