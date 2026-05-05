use crate::solution::Solution;

use rand::{Rng, RngExt};

pub struct SwapTrucks;

impl super::Operator for SwapTrucks {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        for i in 1..(solution.truck_path.len() - 2) {
            for j in i..(solution.truck_path.len() - 1) {
                let mut new_truck_path = solution.truck_path.clone();
                new_truck_path.swap(i, j);
                neighborhood.push(Solution::new(new_truck_path, solution.flights.clone()));
            }
        }

        neighborhood
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        let i = rng.random_range(1..(solution.truck_path.len() - 2));
        let j = rng.random_range(i..(solution.truck_path.len() - 1));

        let mut new_truck_path = solution.truck_path.clone();
        new_truck_path.swap(i, j);

        Some(Solution::new(new_truck_path, solution.flights.clone()))
    }
}
