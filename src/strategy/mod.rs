pub mod random_search;
pub mod local_search;
pub mod simulated_annealing;
pub mod general_adaptive;
pub mod timed_adaptive;

pub use random_search::RandomSearch;
pub use local_search::LocalSearch;
pub use simulated_annealing::SimulatedAnnealing;
pub use general_adaptive::GeneralAdaptive;
pub use timed_adaptive::TimedAdaptive;

pub trait Strategy {
    fn solve(&mut self, problem: &crate::problem::Problem) -> (crate::solution::Solution, u32);
}
