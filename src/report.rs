use crate::operator::{OneInsert, DeployDrone};
use crate::problem::Problem;
use crate::solution::Solution;
use crate::solver::Solver;
use crate::strategy::{LocalSearch, RandomSearch, SimulatedAnnealing, Strategy};

use std::time::Instant;
use rand::rngs::SmallRng;
use rand::SeedableRng;

const RNG_SEED: u64 = 1337;

pub struct InstanceReport {
    instance_name: String,
    strategy_reports: Vec<StrategyReport>,
}

impl InstanceReport {
    pub fn generate(instance_path: &str) -> Self {
        let instance_name = instance_path
            .to_string()
            .split('/')
            .last()
            .unwrap()
            .to_string();

        let mut instance_report = Self {
            instance_name,
            strategy_reports: Vec::new(),
        };

        let problem = Problem::from_file(instance_path);

        let mut random_solver = Solver { 
            strategy: RandomSearch::new(SmallRng::seed_from_u64(RNG_SEED)) 
        };

        instance_report.strategy_reports
            .push(StrategyReport::generate(&mut random_solver, &problem));

        /*
        let mut local_solver = Solver { 
            strategy: LocalSearch::new(SmallRng::seed_from_u64(RNG_SEED))
                .add_operator(OneInsert, 1)
        };

        instance_report.strategy_reports
            .push(StrategyReport::generate(&mut local_solver, &problem));

        let mut sim_annealing_solver = Solver {
            strategy: SimulatedAnnealing::new(SmallRng::seed_from_u64(RNG_SEED))
                .add_operator(DeployDrone, 1)
                .add_operator(OneInsert, 1)
        };

        instance_report.strategy_reports
            .push(StrategyReport::generate(&mut sim_annealing_solver, &problem));
        */

        return instance_report;
    }    

    pub fn print(&self) {
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

        let title_len = len1 + len2 + len3 + len4 + len5 + 14; 

        println!("+{}+", "-".repeat(title_len));
        println!("|{: ^title_len$}|", self.instance_name);
        println!("{}", divider);
        println!("| {: ^len1$} | {} | {} | {} | {} |", col1, col2, col3, col4, col5);
        println!("{}", divider);

        let mut best_solution = Solution::default();
        let mut best_score = f64::MAX;

        for report in self.strategy_reports.iter() {
            if report.best_score < best_score {
                best_solution = report.best_solution.clone();
                best_score = report.best_score;
            }
            println!("| {: ^len1$} | {: ^len2$.2} | {: ^len3$.2} | {: ^len4$.2} | {: ^len5$.3} |",
                report.strategy_name,
                report.average_score,
                report.best_score,
                report.improvement,
                report.average_time
            );
        }
        println!("{}", divider);
        println!("Best solution: {}", best_solution.to_submission_format());
        println!();
    }
}

pub struct StrategyReport {
    pub strategy_name: String,
    pub best_solution: Solution,
    pub best_score: f64,
    pub average_score: f64,
    pub average_time: f64,
    pub improvement: f64,
}

impl StrategyReport {
    pub fn generate<T>(solver: &mut Solver<T>, problem: &Problem) -> Self 
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
            best_score: best_score as f64 / 100.0,
            average_score: average_score as f64 / 100.0,
            average_time,
            improvement,
        }
    }
}
