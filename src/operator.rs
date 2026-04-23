use crate::solution::{Flight, Solution};

pub trait Operator {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution>;
    //fn get_random_neighbor(&self, solution: &Solution, rng: ) -> Solution;
}

pub struct Swap;

impl Operator for Swap {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        for i in 1..(solution.truck_path.len() - 1) {
            for j in (i + 1)..(solution.truck_path.len() - 1) {
                let mut new_truck_path = solution.truck_path.clone();
                new_truck_path.swap(i, j);
                neighborhood.push(
                    Solution::new(new_truck_path, solution.flights.clone())
                );
            }
        }
        return neighborhood;
    }
}

pub struct OneInsert;

impl Operator for OneInsert {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        let n = solution.truck_path.len() + solution.flights.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let mut new_truck_path = solution.truck_path.clone();
                let mut new_flights = solution.flights.clone();

                let node = if i < solution.truck_path.len() {
                    new_truck_path.remove(i)
                } else {
                    new_flights.remove(i).goal
                };

                // NOTE: check if any flight paths depended on the node

                if j < solution.truck_path.len() {
                    new_truck_path.insert(j, node);
                } else {
                    new_flights.insert(j, Flight {
                        start: 0, 
                        goal: node,
                        end: 0,
                    });
                }

                neighborhood.push(
                    Solution::new(new_truck_path, new_flights)
                );
            }
        }

        return neighborhood;
    }
    /*
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        for i in 1..(solution.truck_path.len() - 1) {
            for j in (i + 1)..(solution.truck_path.len() - 1) {
                if i == j { continue; }
                
                let mut new_truck_path = solution.truck_path.clone();
                new_truck_path.swap(i, j);
                neighborhood.push(
                    Solution::new(new_truck_path, solution.flights.clone())
                );
            }
        }
        return neighborhood;
    }
    */
}

