use std::io::{Read};
use std::fs::{File};

#[derive(Debug, Default)]
pub struct TimeMatrix {
    pub rows: usize,
    pub cols: usize,
    pub items: Vec<u32>,
}

impl TimeMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            items: vec![0; rows*cols],
        }
    }

    pub fn get(&self, from: usize, to: usize) -> u32 {
        return self.items[from * self.cols + to];
    }

    pub fn set(&mut self, from: usize, to: usize, value: u32) {
        self.items[from * self.cols + to] = value;
    }
}



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
}

