use crate::solution::{Flight, Solution};

use rand::{Rng, RngExt};

pub struct DeployDrone;

impl DeployDrone {
    fn get_neighbor(&self, solution: &Solution, i: usize) -> Option<Solution> {
        if i == 0 || i > solution.truck_path.len() - 2 {
            return None;
        }

        let mut neighbor = solution.clone();

        let new_flight = Flight {
            start: neighbor.truck_path[i - 1],
            goal: neighbor.truck_path[i],
            end: neighbor.truck_path[i + 1],
        };

        neighbor.truck_path.remove(i);
        neighbor.flights.push(new_flight);

        return Some(neighbor);
    }
}

impl super::Operator for DeployDrone {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        if solution.truck_path.len() < 3 {
            return neighborhood;
        }

        for i in 1..(solution.truck_path.len() - 1) {
            if let Some(neighbor) = self.get_neighbor(solution, i) {
                neighborhood.push(neighbor);
            }
        }

        return neighborhood;
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        if solution.truck_path.len() < 3 {
            return None;
        }

        let i = rng.random_range(1..(solution.truck_path.len() - 1));

        return self.get_neighbor(solution, i);
    }
}
