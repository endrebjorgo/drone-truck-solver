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

    fn solution_drone_deployments_are_valid(&self, solution: &Solution) -> bool {
        // count number of drone departures and arrivals at each node
        let mut departures = vec![0; self.customer_count + 1];
        let mut arrivals = vec![0; self.customer_count + 1];

        for flight in solution.flights.iter() {
            departures[flight.start] += 1;
            arrivals[flight.end] += 1;
        }

        // check that the number of deployed drones is always 0, 1 or 2
        let mut deployed_drones = 0;

        for (idx, &node) in solution.truck_path.iter().enumerate() {
            let node_is_not_last = node != 0 || idx == 0;
            let node_is_not_first = idx != 0;

            if node_is_not_last {
                deployed_drones += departures[node];
            }

            if node_is_not_first {
                deployed_drones -= arrivals[node];
            }

            if deployed_drones < 0 || deployed_drones > 2 {
                return false;
            }
        }

        // no drones are deployed when the tour is over
        return deployed_drones == 0;
    }

    pub fn calculate_score(&self, solution: &Solution) -> Option<u32> {
        if !self.solution_starts_and_ends_at_depot(solution) {
            return None;
        }

        if !self.solution_serves_all_nodes_once(solution) {
            return None;
        }

        if !self.solution_drone_deployments_are_valid(solution) {
            return None;
        }

        if !solution.flights_deploy_in_order() {
            return None;
        }

        let index_lookup = solution.generate_truck_path_index_lookup();

        let (drone1, drone2) = solution.split_flights();

        let mut drone_flights: Vec<Vec<(usize, usize, usize)>> = Vec::new();
        drone_flights.push(drone1.iter().map(|x|
                (x.goal, index_lookup[x.start], index_lookup[x.end])
            ).collect()
        );
        drone_flights.push(drone2.iter().map(|x|
                (x.goal, index_lookup[x.start], index_lookup[x.end])
            ).collect()
        );

        let mut t_arrival: Vec<u32> = vec![0; self.customer_count + 1];
        let mut t_departure: Vec<u32> = vec![0; self.customer_count + 1];
        let mut drone_availability: Vec<u32> = vec![0; drone_flights.len()];
        let mut total_time: u32 = 0;

        for i in 1..solution.truck_path.len() {
            let prev_node = solution.truck_path[i - 1];
            let curr_node = solution.truck_path[i];

            // Truck travel time
            let truck_travel = self.truck_times.get(prev_node, curr_node);
            let truck_arrival = t_departure[prev_node] + truck_travel;
            t_arrival[curr_node] = truck_arrival;

            // Check returning drones at curr_node (index i)
            let mut drone_returns: Vec<u32> = Vec::new();

            for (u, flights) in drone_flights.iter().enumerate() {
                for &(cust, launch_idx, return_idx) in flights {
                    if return_idx == i {
                        let launch_node = solution.truck_path[launch_idx];
                        let return_node = solution.truck_path[return_idx];
                        let flight_out = self.drone_times.get(launch_node, cust);
                        let flight_back = self.drone_times.get(cust, return_node);
                        let total_flight = flight_out + flight_back;

                        // Drone cannot depart before both truck and its own availability
                        let possible_launch_time = t_arrival[launch_node];
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
                            t_arrival[curr_node] > drone_return_time {
                                t_arrival[curr_node] - drone_return_time
                            } else {
                                0
                            };
                        let total_flight_with_wait = total_flight + drone_wait;
                        if total_flight_with_wait > self.flight_limit {
                            return None;
                        }
                    }
                }
            }
            //
            // Truck waits for the latest returning drone
            if !drone_returns.is_empty() {
                let latest_drone = drone_returns.iter().fold(0, |a, &b| a.max(b));
                t_departure[curr_node] = cmp::max(truck_arrival, latest_drone);
            } else {
                t_departure[curr_node] = truck_arrival;
            }

            // Add truck arrival time to total_time if not depot
            if curr_node != 0 {
                total_time += truck_arrival;
            }
        }

        return Some(total_time);
    }

    /*
    pub fn calculate_score(&self, solution: &Solution) -> Option<u32> {
        // TODO: incorporate checks into calculation to avoid redundant loops
        // specifically the ones that loop

        if !self.solution_starts_and_ends_at_depot(solution) {
            return None;
        }

        if !self.solution_serves_all_nodes_once(solution) {
            return None;
        }

        if !self.solution_drone_deployments_are_valid(solution) {
            return None;
        }

        if !solution.flights_deploy_in_order() {
            return None;
        }

        let mut total_time: u32 = 0;
        let mut curr_time: u32 = 0;

        let mut drone_arrivals: Vec<u32> = vec![0; self.customer_count + 1];
        let mut truck_arrivals: Vec<u32> = vec![0; self.customer_count + 1];

        let mut prev_node: usize = 0;

        assert_eq!(solution.truck_path.first(), Some(&0));

        for &curr_node in solution.truck_path.iter() {
            // arrival time of truck at current node (0 if starting)
            curr_time += self.truck_times.get(prev_node, curr_node);
            truck_arrivals[curr_node] = curr_time;

            // add the current time to the total time, as the truck has arrived
            total_time += curr_time;

            // the truck checks if it can send out 0/1/2 drone(s).
            for flight in solution.flights.iter() {
                if flight.start != curr_node {
                    continue;
                }

                let out_time = self.drone_times.get(flight.start, flight.goal);
                let in_time = self.drone_times.get(flight.goal, flight.end);

                // does the flight time exceed limit?
                if out_time + in_time > self.flight_limit {
                    return None;
                }

                // add arrival time at goal node
                total_time += curr_time + out_time;
                drone_arrivals[flight.goal] = curr_time + out_time;

                // save maximum arrival time of drones
                let drone_arrival = curr_time + out_time + in_time;

                if drone_arrival > drone_arrivals[flight.end] {
                    drone_arrivals[flight.end] = drone_arrival;
                }
            }

            // NOTE: it may not be best to send out drones instantly if the 
            // truck is waiting for another drone to arrive before departing. 
            // this may cause the total flight time to exceed limit, but it 
            // does however reduce the arrival time at the goal nodes.

            // check if truck must wait on some arriving drone
            if curr_time < drone_arrivals[curr_node] {
                curr_time = drone_arrivals[curr_node];
            }

            prev_node = curr_node;
        }

        // check if the truck ever takes longer to travel than the flight limit:
        for flight in solution.flights.iter() {
            let truck_duration = if flight.start != 0 {
                truck_arrivals[flight.end] - truck_arrivals[flight.start]
            } else {
                truck_arrivals[flight.end]
            };

            if truck_duration > self.flight_limit {
                return None;
            }
        }

        return Some(total_time);
    }
*/
}

