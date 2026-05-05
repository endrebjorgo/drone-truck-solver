use crate::solution::{Flight, Solution};

use rand::{Rng, RngExt};

pub struct DeployDrone;

impl super::Operator for DeployDrone {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        if solution.truck_path.len() < 3 {
            return neighborhood;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();

        for i in 1..(solution.truck_path.len() - 1) {
            let mut new_truck_path = solution.truck_path.clone();
            let mut new_flights = solution.flights.clone();

            let new_flight = Flight {
                start: solution.truck_path[i - 1],
                goal: solution.truck_path[i],
                end: solution.truck_path[i + 1],
            };

            new_truck_path.remove(i);

            let insert_idx = new_flights
                .iter()
                .position(|flight| index_lookup[flight.start] > index_lookup[new_flight.start])
                .unwrap_or(new_flights.len());

            new_flights.insert(insert_idx, new_flight);

            neighborhood.push(Solution::new(new_truck_path, new_flights));
        }

        neighborhood
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        if solution.truck_path.len() < 3 {
            return None;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();
        let i = rng.random_range(1..(solution.truck_path.len() - 1));

        let mut new_truck_path = solution.truck_path.clone();
        let mut new_flights = solution.flights.clone();

        let new_flight = Flight {
            start: solution.truck_path[i - 1],
            goal: solution.truck_path[i],
            end: solution.truck_path[i + 1],
        };

        new_truck_path.remove(i);

        let insert_idx = new_flights
            .iter()
            .position(|flight| index_lookup[flight.start] > index_lookup[new_flight.start])
            .unwrap_or(new_flights.len());

        new_flights.insert(insert_idx, new_flight);

        Some(Solution::new(new_truck_path, new_flights))
    }
}
