use crate::solution::Solution;

use rand::{Rng, RngExt};

pub struct ScoochLaunchAndLanding;

impl super::Operator for ScoochLaunchAndLanding {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        if solution.flights.len() == 0 {
            return neighborhood;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();

        for i in 0..solution.flights.len() {
            let start_idx = index_lookup[solution.flights[i].start];
            let end_idx = index_lookup[solution.flights[i].end];

            for j in 0..2 {
                match j {
                    0 => {
                        if start_idx == 0 {
                            continue;
                        }

                        let mut new_flights = solution.flights.clone();
                        new_flights[i].start = solution.truck_path[start_idx - 1];
                        neighborhood.push(Solution::new(solution.truck_path.clone(), new_flights));
                    }
                    1 => {
                        if solution.truck_path[end_idx + 1] == 0 {
                            continue;
                        }

                        let mut new_flights = solution.flights.clone();
                        new_flights[i].end = solution.truck_path[end_idx + 1];
                        neighborhood.push(Solution::new(solution.truck_path.clone(), new_flights));
                    }
                    _ => unimplemented!(),
                }
            }
        }

        neighborhood
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        if solution.flights.len() == 0 {
            return None;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();
        let i = rng.random_range(0..solution.flights.len());
        let j = rng.random_range(0..2);

        let start_idx = index_lookup[solution.flights[i].start];
        let end_idx = index_lookup[solution.flights[i].end];

        match j {
            0 => {
                if start_idx == 0 {
                    return None;
                }

                let mut new_flights = solution.flights.clone();
                new_flights[i].start = solution.truck_path[start_idx - 1];
                Some(Solution::new(solution.truck_path.clone(), new_flights))
            }
            1 => {
                if solution.truck_path[end_idx + 1] == 0 {
                    return None;
                }

                let mut new_flights = solution.flights.clone();
                new_flights[i].end = solution.truck_path[end_idx + 1];
                Some(Solution::new(solution.truck_path.clone(), new_flights))
            }
            _ => unimplemented!(),
        }
    }
}
