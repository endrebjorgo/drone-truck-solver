use crate::solution::Solution;

use rand::{Rng, RngExt};

pub struct TwoOpt;

impl TwoOpt {
    fn get_neighbor(&self, solution: &Solution, i: usize, j: usize) -> Option<Solution> {
        if i >= j {
            return None;
        }

        if i == 0 || i >= solution.truck_path.len() {
            return None;
        }

        if j == 0 || j >= solution.truck_path.len() {
            return None;
        }

        let mut neighbor = solution.clone();

        neighbor.truck_path[i..=j].reverse();

        let lookup = neighbor.generate_truck_path_index_lookup();

        for flight in neighbor.flights.iter_mut() {
            if lookup[flight.start] > lookup[flight.end] {
                std::mem::swap(&mut flight.start, &mut flight.end);
            }
        }
        
        return Some(neighbor);
    }
}

impl super::Operator for TwoOpt {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        for i in 1..(solution.truck_path.len() - 2) {
            for j in i..(solution.truck_path.len() - 1) {
                if let Some(neighbor) = self.get_neighbor(solution, i, j) {
                    neighborhood.push(neighbor);
                }
            }
        }

        return neighborhood;
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        let i = rng.random_range(1..(solution.truck_path.len() - 2));
        let j = rng.random_range(i..(solution.truck_path.len() - 1));

        return self.get_neighbor(solution, i, j);
    }
}
