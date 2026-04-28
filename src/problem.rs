use crate::solution::{Flight, Solution};
use crate::time_matrix::TimeMatrix;

use std::cmp;
use std::io::{Read};
use std::fs::{File};
use rand::prelude::SliceRandom;
use rand::RngExt;

// TODO: implement a drone count
#[derive(Debug, Default)]
pub struct Problem {
    pub customer_count: usize,
    pub flight_limit: u32,
    pub truck_times: TimeMatrix,
    pub drone_times: TimeMatrix,
}

impl Problem {
    pub fn from_file(file_path: &str) -> Self {
        let mut problem = Self::default();

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);

        let mut lines = buffer.lines();

        assert_eq!(lines.next().unwrap(), "# Number of customers");

        problem.customer_count = lines.next().unwrap().parse().unwrap();
        
        problem.truck_times = TimeMatrix::new(
            problem.customer_count + 1, problem.customer_count + 1
        );

        problem.drone_times = TimeMatrix::new(
            problem.customer_count + 1, problem.customer_count + 1
        );

        assert_eq!(lines.next().unwrap(), "# Drone flight limit");

        problem.flight_limit = lines.next().unwrap().parse().unwrap();

        assert_eq!(lines.next().unwrap(), "# Travel time matrix for the truck");

        let mut curr_row: Vec<u32>;
        for row in 0..=problem.customer_count {
            curr_row = lines.next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.trim_end_matches(".0").parse().unwrap())
                .collect();

            assert_eq!(curr_row.len(), problem.customer_count + 1);

            for col in 0..=problem.customer_count {
                problem.truck_times.set(row, col, curr_row[col]);
            }
        }

        assert_eq!(lines.next().unwrap(), "# Travel time matrix for the drones");

        for row in 0..=problem.customer_count {
            curr_row = lines.next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.trim_end_matches(".0").parse().unwrap())
                .collect();

            assert_eq!(curr_row.len(), problem.customer_count + 1);

            for col in 0..=problem.customer_count {
                problem.drone_times.set(row, col, curr_row[col]);
            }
        }

        assert_eq!(lines.next().unwrap(), "#");

        return problem;
    }

    pub fn generate_initial_solution(&self) -> Solution {
        let mut truck_path: Vec<usize> = (0..=self.customer_count).collect();
        truck_path.push(0);
        Solution::new(truck_path, Vec::new())
    }

    pub fn generate_with_heuristic(&self) -> Solution {
        let mut truck_path: Vec<usize> = vec![0];
        let mut flights: Vec<Flight> = Vec::new();

        let mut unvisited: Vec<usize> = (1..=self.customer_count).collect();
        while !unvisited.is_empty() {
            let prev = *truck_path.last().unwrap();

            let next = unvisited.iter()
                .min_by_key(|&&node| self.truck_times.get(prev, node))
                .copied()
                .unwrap();

            truck_path.push(next);
            unvisited.retain(|&x| x != next);
        }
        truck_path.push(0);

        let mut i = 1;
        while i < truck_path.len() - 2 {
            let prev = truck_path[i - 1];
            let curr = truck_path[i];
            let next = truck_path[i + 1];

            let drone_time  = self.drone_times.get(prev, curr) + self.drone_times.get(curr, next);

            if drone_time > self.flight_limit {
                i += 1;
                continue;
            }

            let old_truck_time = self.truck_times.get(prev, curr) + self.truck_times.get(prev, curr);
            let new_truck_time = self.truck_times.get(prev, next);

            if cmp::max(drone_time, new_truck_time) > self.flight_limit {
                i += 1;
                continue;
            }

            if cmp::max(drone_time, new_truck_time) > old_truck_time {
                i += 1;
                continue;
            }

            truck_path.remove(i);
            flights.push(Flight {
                start: prev,
                goal: curr,
                end: next
            });
            i += 1;
        }

        return Solution::new(truck_path, flights);
    }

    pub fn generate_random_solution(&self, rng: &mut impl rand::Rng) -> Solution {
        let mut all_nodes: Vec<usize> = (1..=self.customer_count).collect();
        all_nodes.shuffle(rng);

        let (truck_nodes, drone_nodes) = all_nodes.split_at(
            rng.random_range(1..=all_nodes.len())
        );

        let mut truck_path = vec![0];
        truck_path.append(&mut truck_nodes.to_vec());
        truck_path.push(0);

        let mut flights = Vec::new();

        for &goal in drone_nodes.iter() {
            // generate two random idxs which are nodes in correct order
            let start_idx = rng.random_range(0..truck_path.len() - 1);
            let end_idx = rng.random_range((start_idx+1)..truck_path.len());
            let start = truck_path[start_idx];
            let end = truck_path[end_idx];
            flights.push( Flight { start, goal, end });
        }

        Solution::new(truck_path, flights)
    }

    fn solution_starts_and_ends_at_depot(&self, solution: &Solution) -> bool {
        return solution.truck_path.first() == Some(&0) 
            && solution.truck_path.last() == Some(&0);
    }

    fn solution_serves_all_nodes_once(&self, solution: &Solution) -> bool {
        let required_len = solution.truck_path.len() + solution.flights.len();
        if self.customer_count + 2 != required_len {
            return false;
        }

        let mut counts = vec![1; self.customer_count + 1];
        counts[0] = 2;

        for &node in solution.truck_path.iter() {
            if counts[node] == 0 {
                return false;
            }
            counts[node] -= 1; 
        }

        for flight in solution.flights.iter() {
            if counts[flight.goal] == 0 {
                return false;
            }
            counts[flight.goal] -= 1; 
        }
        
        return true;
    }

    pub fn solution_flights_are_feasible(&self, solution: &Solution) -> bool {
        let index_lookup = solution.generate_truck_path_index_lookup();

        for f in solution.flights.iter() {
            if index_lookup[f.start] >= index_lookup[f.end] {
                return false;
            }

            if f.start == f.goal {
                return false;
            }

            if f.end == f.goal {
                return false;
            }

            if self.drone_times.get(f.start, f.goal) + self.drone_times.get(f.goal, f.end) > self.flight_limit {
                return false;
            }
        }

        return true;
    }

    pub fn calculate_score(&self, solution: &Solution) -> Option<u32> {
        if !self.solution_starts_and_ends_at_depot(solution) {
            return None;
        }

        if !self.solution_serves_all_nodes_once(solution) {
            return None;
        }

        if !self.solution_flights_are_feasible(solution) {
            return None;
        }

        // returns None if unable to split, i.e. flights overlap
        let (drone1, drone2) = solution.split_flights().ok()?;

        let index_lookup = solution.generate_truck_path_index_lookup();

        let drone_flights: Vec<Vec<(usize, usize, usize)>> = vec![
            drone1.iter().map(|x| (x.goal, index_lookup[x.start], index_lookup[x.end])).collect(),
            drone2.iter().map(|x| (x.goal, index_lookup[x.start], index_lookup[x.end])).collect(),
        ];

        let mut arrival_times: Vec<u32> = vec![0; self.customer_count + 1];
        let mut departure_times: Vec<u32> = vec![0; self.customer_count + 1];
        let mut drone_availability: Vec<u32> = vec![0, 0];
        let mut total_time: u32 = 0;

        for i in 1..solution.truck_path.len() {
            let prev_node = solution.truck_path[i - 1];
            let curr_node = solution.truck_path[i];

            // Truck travel time
            let truck_travel = self.truck_times.get(prev_node, curr_node);
            let truck_arrival = departure_times[prev_node] + truck_travel;
            arrival_times[curr_node] = truck_arrival;

            // Check returning drones at curr_node (index i)
            let mut drone_returns: Vec<u32> = Vec::new();

            for (u, flights) in drone_flights.iter().enumerate() {
                for &(cust, launch_idx, return_idx) in flights {
                    if return_idx != i {
                        continue;
                    }
                    let launch_node = solution.truck_path[launch_idx];
                    let return_node = solution.truck_path[return_idx];
                    let flight_out = self.drone_times.get(launch_node, cust);
                    let flight_back = self.drone_times.get(cust, return_node);
                    let total_flight = flight_out + flight_back;

                    // Drone cannot depart before both truck and its own availability
                    let possible_launch_time = arrival_times[launch_node];
                    let actual_launch_time = if launch_node == 0 {
                        0
                    } else {
                        cmp::max(possible_launch_time, drone_availability[u])
                    };

                    let drone_arrival_customer = actual_launch_time + flight_out;
                    let drone_return_time = actual_launch_time + total_flight;
                    drone_availability[u] = drone_return_time;
                    drone_returns.push(drone_return_time);
                    total_time += drone_arrival_customer;

                    // Check flight range
                    let drone_wait = if curr_node != 0 && 
                        arrival_times[curr_node] > drone_return_time {
                            arrival_times[curr_node] - drone_return_time
                        } else {
                            0
                        };
                    let total_flight_with_wait = total_flight + drone_wait;
                    if total_flight_with_wait > self.flight_limit {
                        return None;
                    }
                }
            }

            // Truck waits for the latest returning drone
            if !drone_returns.is_empty() {
                let latest_drone = drone_returns.iter().fold(0, |a, &b| a.max(b));
                departure_times[curr_node] = cmp::max(truck_arrival, latest_drone);
            } else {
                departure_times[curr_node] = truck_arrival;
            }

            // Add truck arrival time to total_time if not depot
            if curr_node != 0 {
                total_time += truck_arrival;
            }
        }

        return Some(total_time);
    }

    pub fn remove_moot_nodes(&self, solution: &Solution) -> Option<Solution> {
        let index_lookup = solution.generate_truck_path_index_lookup();

        let mut new_truck_path = solution.truck_path.clone();
        let mut new_flights = solution.flights.clone();

        let mut i = 0;
        while i < new_flights.len() {
            let flight = &new_flights[i];

            let start_idx = index_lookup[new_flights[i].start];
            let end_idx = index_lookup[new_flights[i].end];
            let next_idx = start_idx + 1;
            let next_node = solution.truck_path[next_idx];

            let mut new_approx_time = self.truck_times.get(flight.start, flight.goal)
                + self.truck_times.get(flight.goal, next_node);

            for j in next_idx..end_idx {
                new_approx_time += self.truck_times.get(solution.truck_path[j], solution.truck_path[j+1]);
            }

            let curr_drone_time = self.drone_times.get(flight.start, flight.goal) 
                + self.drone_times.get(flight.goal, flight.end);

            if curr_drone_time > new_approx_time {
                new_truck_path.insert(next_idx, flight.goal);
                new_flights.remove(i);
            } else {
                i += 1;
            }
        }
        let new_solution = Solution::new(new_truck_path, new_flights);
        if self.calculate_score(&new_solution).is_some() {
            return Some(new_solution);
        } else {
            return None;
        }
    }
}

