use crate::solution::{Flight, Solution};

use rand::{Rng, RngExt};

pub struct OneInsert;

impl super::Operator for OneInsert {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        if solution.truck_path.len() < 4 {
            return neighborhood;
        }

        let n = solution.truck_path.len() + solution.flights.len();

        for i in 1..n {
            for j in 1..n {
                if i + 1 == solution.truck_path.len() {
                    continue;
                }

                if i + 2 == solution.truck_path.len() {
                    continue;
                }

                if i == j {
                    continue;
                }

                let mut new_truck_path = solution.truck_path.clone();
                let mut new_flights = solution.flights.clone();

                let node = if i < solution.truck_path.len() {
                    new_truck_path.remove(i)
                } else {
                    new_flights.remove(i - solution.truck_path.len()).goal
                };

                if j < solution.truck_path.len() {
                    new_truck_path.insert(j, node);
                } else {
                    if i < solution.truck_path.len() {
                        new_flights.insert(
                            j - solution.truck_path.len(),
                            Flight {
                                start: solution.truck_path[i - 1],
                                goal: node,
                                end: solution.truck_path[i + 1],
                            },
                        );
                    } else {
                        new_flights.insert(
                            j - solution.truck_path.len(),
                            solution.flights[i - solution.truck_path.len()].clone(),
                        );
                    }
                }

                neighborhood.push(Solution::new(new_truck_path, new_flights));
            }
        }

        neighborhood
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        let n = solution.truck_path.len() + solution.flights.len();

        if solution.truck_path.len() < 4 {
            return None;
        }

        let mut i = rng.random_range(1..n);
        let mut j = rng.random_range(1..n);

        while i + 1 == solution.truck_path.len() || i + 2 == solution.truck_path.len() {
            i = rng.random_range(1..n);
        }

        while i == j {
            j = rng.random_range(0..n);
        }

        let mut new_truck_path = solution.truck_path.clone();
        let mut new_flights = solution.flights.clone();

        let node = if i < solution.truck_path.len() {
            new_truck_path.remove(i)
        } else {
            new_flights.remove(i - solution.truck_path.len()).goal
        };

        if j < solution.truck_path.len() {
            new_truck_path.insert(j, node);
        } else {
            if i < solution.truck_path.len() {
                new_flights.insert(
                    j - solution.truck_path.len(),
                    Flight {
                        start: solution.truck_path[i - 1],
                        goal: node,
                        end: solution.truck_path[i + 1],
                    },
                );
            } else {
                new_flights.insert(
                    j - solution.truck_path.len(),
                    solution.flights[i - solution.truck_path.len()].clone(),
                );
            }
        }

        Some(Solution::new(new_truck_path, new_flights))
    }
}
