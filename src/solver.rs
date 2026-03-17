use crate::problem::Problem;
use crate::solution::{Flight, Solution};

pub enum Objective {
    Minimize,
    Maximize,
}

pub struct Solver {
    pub problem: Problem,
    pub objective: Objective,
}

impl Solver {
    pub fn minimize(problem: Problem) -> Self {
        Self {
            problem,
            objective: Objective::Minimize,
        }
    }

    pub fn maximize(problem: Problem) -> Self {
        Self {
            problem,
            objective: Objective::Maximize,
        }
    }

    pub fn is_better_score(&self, candidate: u32, current: u32) -> bool {
        match self.objective {
            Objective::Maximize => candidate > current,
            Objective::Minimize => candidate < current,
        }
    }

    pub fn generate_initial_solution(&self) -> Solution {
        let mut truck_path: Vec<usize> = (0..=self.problem.customer_count).collect();
        truck_path.push(0);

        Solution {
            truck_path,
            flights1: Vec::new(),
            flights2: Vec::new(),
        }
    }

    /*
    pub fn calculate_score(&self, solution: &Solution) -> u32 {
        let mut score: u32 = 0;

        let mut cumulative_truck_time = vec![0u32];
        let mut accumulator = 0;
        for pair in solution.truck_path.windows(2) {
            accumulator += self.truck_times.get(pair[0], pair[1]);
            cumulative_truck_time.push(accumulator);
        }

        score += cumulative_truck_time.last().unwrap();

        let mut total_drone_time = 0;

        for (i, &start_idx) in solution.drone_starts.iter().enumerate() {
            if start_idx == 0 {
                continue;
            }

            let start_node = solution.truck_path[start_idx - 1];
            let goal_node = solution.drone_paths[i];
            let end_node = solution.truck_path[solution.drone_ends[i]];
            let truck_time = self.truck_times.get(start_node, end_node);

            total_drone_time += self.drone_times.get(start_node, goal_node);
            total_drone_time += cumulative_truck_time[start_idx - 1];
        }
        return score; 
    }

    pub fn random_solution(&self) -> Solution {
        let mut solution = Solution::default();

        let mut rng = rand::rngs::SmallRng::seed_from_u64(RNG_SEED);

        let mut nodes: Vec<usize> = (1..=self.customer_count).collect();
        nodes.shuffle(&mut rng);

        let (truck_path, drone_paths) = nodes.split_at(
            rng.random_range(1..=nodes.len())
        );

        let (drone_path_1, drone_path_2) = drone_paths.split_at(
            rng.random_range(0..=drone_paths.len())
        );

        solution.truck_path.push(0);
        solution.truck_path.append(&mut truck_path.to_vec());
        solution.truck_path.push(0);

        solution.drone_paths.append(&mut drone_path_1.to_vec());
        solution.drone_paths.push(0);
        solution.drone_paths.append(&mut drone_path_2.to_vec());

        solution.drone_starts = solution.drone_paths;

        // starts and ends have same shape as drone path. position idx in starts 
        // must be less than idx in ends


        // TODO: remove this
        solution = self.generate_initial_solution();

        return solution;
    }
    */
}
