use crate::solution::Solution;

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
}

pub struct OneInsert;

impl Operator for OneInsert {
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
}

