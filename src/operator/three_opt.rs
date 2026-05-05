use crate::solution::Solution;

use rand::{Rng, RngExt};

pub struct ThreeOpt;

impl ThreeOpt {
    fn get_neighbor(
        &self, 
        solution: &Solution, 
        i: usize, 
        j: usize, 
        k: usize, 
        flip1: bool, 
        flip2: bool,
        switch: bool
    ) -> Option<Solution> {
        if i >= j {
            return None;
        }
        if j >= k {
            return None;
        }

        if i == 0 || i >= solution.truck_path.len() - 2 {
            return None;
        }

        if j == 0 || j >= solution.truck_path.len() - 1 {
            return None;
        }

        if k == 0 || k >= solution.truck_path.len() {
            return None;
        }

        if !(flip1 || flip2 || switch) {
            return None;
        }

        let mut neighbor = solution.clone();

        if flip1 {
            neighbor.truck_path[i..=j].reverse();
        }

        if flip2 {
            neighbor.truck_path[j..=k].reverse();
        }

        if switch {
            neighbor.truck_path[i..=k].reverse();
            if flip1 {
                neighbor.truck_path[i..=j].reverse();
            }
            if flip2 {
                neighbor.truck_path[j..=k].reverse();
            }
        }

        let lookup = neighbor.generate_truck_path_index_lookup();

        for flight in neighbor.flights.iter_mut() {
            if lookup[flight.start] > lookup[flight.end] {
                std::mem::swap(&mut flight.start, &mut flight.end);
            }
        }
        
        return Some(neighbor);
    }
}

impl super::Operator for ThreeOpt {
    fn generate_neighborhood(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighborhood: Vec<Solution> = Vec::new();

        // yuck...
        for i in 1..(solution.truck_path.len() - 3) {
            for j in i..(solution.truck_path.len() - 2) {
                for k in j..(solution.truck_path.len() - 1) {
                    for &f1 in [false, true].iter() {
                        for &f2 in [false, true].iter() {
                            for &s in [false, true].iter() {
                                if let Some(neighbor) = self.get_neighbor(solution, i, j, k, f1, f2, s) {
                                    neighborhood.push(neighbor);
                                }
                            }
                        }
                    }
                }
            }
        }

        return neighborhood;
    }

    fn get_random_neighbor(&self, solution: &Solution, rng: &mut dyn Rng) -> Option<Solution> {
        let i = rng.random_range(1..(solution.truck_path.len() - 3));
        let j = rng.random_range(i..(solution.truck_path.len() - 2));
        let k = rng.random_range(i..(solution.truck_path.len() - 1));
        let f1 = rng.random_bool(0.5);
        let f2 = rng.random_bool(0.5);
        let s = rng.random_bool(0.5);

        return self.get_neighbor(solution, i, j, k, f1, f2, s);
    }
}

