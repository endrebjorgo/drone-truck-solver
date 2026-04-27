use crate::solution::{Flight, Solution};

use rand::{Rng, RngExt};

pub trait Operator {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution>;
    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution>;
}

pub struct OneInsert;

impl Operator for OneInsert {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        let n = solution.truck_path.len() + solution.flights.len();

        for i in 1..n {
            for j in 1..n {
                if i == solution.truck_path.len() {
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
                    new_flights.remove(i).goal
                };

                if j < solution.truck_path.len() {
                    new_truck_path.insert(j, node);
                } else {
                    if i < solution.truck_path.len() {
                        new_flights.insert(j, Flight {
                            start: solution.truck_path[i - 1], 
                            goal: node,
                            end: solution.truck_path[i + 1],
                        });
                    } else {
                        new_flights.insert(j, solution.flights[i].clone());
                    }
                }

                neighborhood.push(
                    Solution::new(new_truck_path, new_flights)
                );
            }
        }

        return neighborhood;
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        let n = solution.truck_path.len() + solution.flights.len();

        let mut i = rng.random_range(1..n);
        let mut j = rng.random_range(1..n);

        // reroll if invalid random value
        while i == solution.truck_path.len() {
            i = rng.random_range(1..n)
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
                new_flights.insert(j - solution.truck_path.len(), Flight {
                    start: solution.truck_path[i - 1], 
                    goal: node,
                    end: solution.truck_path[i + 1],
                });
            } else {
                new_flights.insert(j, solution.flights[i].clone());
            }
        }

        return Some(Solution::new(new_truck_path, new_flights));
    }
}

pub struct DeployDrone;

impl Operator for DeployDrone {
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
                start: solution.truck_path[i-1],
                goal: solution.truck_path[i],
                end: solution.truck_path[i+1],
            };

            new_truck_path.remove(i);
            
            let insert_idx = new_flights.iter()
                .position(|flight| index_lookup[flight.start] > index_lookup[new_flight.start])
                .unwrap_or(new_flights.len());

            new_flights.insert(insert_idx, new_flight);

            neighborhood.push(
                Solution::new(new_truck_path, new_flights)
            );
        }

        return neighborhood;
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
            start: solution.truck_path[i-1],
            goal: solution.truck_path[i],
            end: solution.truck_path[i+1],
        };

        new_truck_path.remove(i);
        
        let insert_idx = new_flights.iter()
            .position(|flight| index_lookup[flight.start] > index_lookup[new_flight.start])
            .unwrap_or(new_flights.len());

        new_flights.insert(insert_idx, new_flight);

        return Some(Solution::new(new_truck_path, new_flights));
    }
}

pub struct ScoochLaunchAndLanding;

impl Operator for ScoochLaunchAndLanding {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        if solution.flights.len() == 0 {
            return neighborhood;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();

        for i in 0..solution.flights.len() {
            let mut new_flights;

            let start_idx = index_lookup[solution.flights[i].start];
            let end_idx = index_lookup[solution.flights[i].end];
            
            for j in 0..4 {
                match j {
                    0 => {
                        new_flights = solution.flights.clone();
                        new_flights[i].start = start_idx - 1;
                        neighborhood.push(
                            Solution::new(solution.truck_path.clone(), new_flights)
                        );
                    },
                    1 => {
                        new_flights = solution.flights.clone();
                        new_flights[i].start = start_idx + 1;
                        neighborhood.push(
                            Solution::new(solution.truck_path.clone(), new_flights)
                        );
                    },
                    2 => {
                        new_flights = solution.flights.clone();
                        new_flights[i].end = end_idx - 1;
                        neighborhood.push(
                            Solution::new(solution.truck_path.clone(), new_flights)
                        );
                    },
                    3 => {
                        new_flights = solution.flights.clone();
                        new_flights[i].end = end_idx + 1;
                        neighborhood.push(
                            Solution::new(solution.truck_path.clone(), new_flights)
                        );
                    },
                    _ => unimplemented!(),
                }
            }
        }

        return neighborhood;
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        if solution.flights.len() == 0 {
            return None;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();

        let i = rng.random_range(0..solution.flights.len());
        let j = rng.random_range(0..4);

        let mut new_flights;

        let start_idx = index_lookup[solution.flights[i].start];
        let end_idx = index_lookup[solution.flights[i].end];

        match j {
            0 => {
                new_flights = solution.flights.clone();
                new_flights[i].start = start_idx - 1;
                return Some(Solution::new(solution.truck_path.clone(), new_flights));
            },
            1 => {
                new_flights = solution.flights.clone();
                new_flights[i].start = start_idx + 1;
                return Some(Solution::new(solution.truck_path.clone(), new_flights));
            },
            2 => {
                new_flights = solution.flights.clone();
                new_flights[i].end = end_idx - 1;
                return Some(Solution::new(solution.truck_path.clone(), new_flights));
            },
            3 => {
                new_flights = solution.flights.clone();
                new_flights[i].end = end_idx + 1;
                return Some(Solution::new(solution.truck_path.clone(), new_flights));
            },
            _ => unimplemented!(),
        }
    }
}


pub struct SwapTrucks;

impl Operator for SwapTrucks {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        for i in 1..(solution.truck_path.len() - 2) {
            for j in i..(solution.truck_path.len() - 1) {
                let mut new_truck_path = solution.truck_path.clone();
                new_truck_path.swap(i, j);
            
                neighborhood.push(
                    Solution::new(new_truck_path, solution.flights.clone())
                );
            }
        }

        return neighborhood;
    }
    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        let i = rng.random_range(1..(solution.truck_path.len() - 2));
        let j = rng.random_range(i..(solution.truck_path.len() - 1));

        let mut new_truck_path = solution.truck_path.clone();
        new_truck_path.swap(i, j);
    
        return Some(Solution::new(new_truck_path, solution.flights.clone()));
    }
}
