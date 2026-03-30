use drone_truck_solver::operator::{Swap};
use drone_truck_solver::problem::Problem;
use drone_truck_solver::solution::Solution;
use drone_truck_solver::solver::Solver;
use drone_truck_solver::strategy::{LocalSearch, RandomSearch, Strategy};

use std::time::Instant;
use rand::rngs::SmallRng;
use rand::SeedableRng;

const RNG_SEED: u64 = 1337;

const INPUT_FILES: [&'static str; 8] = [
    "./assets/F_10.txt",
    "./assets/F_100.txt",
    "./assets/F_20.txt",
    "./assets/F_50.txt",
    "./assets/R_10.txt",
    "./assets/R_100.txt",
    "./assets/R_20.txt",
    "./assets/R_50.txt",
];

struct TestReport {
    strategy_name: String,
    best_solution: Solution,
    best_score: u32,
    average_score: u32,
    average_time: f64,
    improvement: f64,
}

impl TestReport {
    fn generate<T>(solver: &mut Solver<T>, problem: &Problem) -> Self 
    where T: Strategy
    {
        let initial_solution = problem.generate_initial_solution();
        let initial_score = problem.calculate_score(&initial_solution)
            .expect("initial solution unexpectedly invalid");
        
        let mut best_solution = initial_solution.clone();
        let mut best_score = initial_score;
        let mut total_score = 0;

        let now = Instant::now();

        for _ in 0..10 {
            let (solution, score) = solver.solve(&problem);

            if score < best_score {
                best_solution = solution;
                best_score = score;
            }

            total_score += score;
        }

        let elapsed = now.elapsed();
        let average_time = elapsed.as_secs_f64() / 10.0;

        let average_score = total_score / 10;
        let improvement = 100.0 * (initial_score as f64 - best_score as f64) / initial_score as f64;

        let strategy_name = std::any::type_name_of_val(&solver.strategy)
            .to_string()
            .split(':')
            .last()
            .unwrap()
            .to_string();

        Self {
            strategy_name,
            best_solution,
            best_score,
            average_score,
            average_time,
            improvement,
        }
    }

    fn print(&self) {
        let col1 = "Strategy";
        let col2 = "Average objective";
        let col3 = "Best objective";
        let col4 = "Improvement (%)";
        let col5 = "Average runtime (s)";

        let len1 = col1.len().max(20);
        let len2 = col2.len();
        let len3 = col3.len();
        let len4 = col4.len();
        let len5 = col5.len();

        let divider = format!("+{}+{}+{}+{}+{}+",
            "-".repeat(len1 + 2),
            "-".repeat(len2 + 2),
            "-".repeat(len3 + 2),
            "-".repeat(len4 + 2),
            "-".repeat(len5 + 2)
        );

        println!("{}", divider);
        println!("| {: ^len1$} | {} | {} | {} | {} |", col1, col2, col3, col4, col5);
        println!("{}", divider);
        println!("| {: ^len1$} | {: ^len2$} | {: ^len3$} | {: ^len4$.2} | {: ^len5$.3} |",
            self.strategy_name,
            self.average_score,
            self.best_score,
            self.improvement,
            self.average_time
        );
        println!("{}", divider);
        println!();

        //println!("Best solution: {}", self.best_solution.to_submission_format());
    }
}

fn main() {
    let problem = Problem::from_file(INPUT_FILES[0]);

    let mut solver = Solver {
        strategy: RandomSearch::new(SmallRng::seed_from_u64(RNG_SEED)),
    };

    let mut solver2 = Solver {
        strategy: LocalSearch::new().add_operator(Swap)
    };

    let report = TestReport::generate(&mut solver, &problem);
    report.print();

    let report2 = TestReport::generate(&mut solver2, &problem);
    report2.print();
}
