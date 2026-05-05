pub mod deploy_drone;
pub mod one_insert;
pub mod scooch_launch_and_landing;
pub mod swap_trucks;
pub mod two_opt;
pub mod three_opt;

pub use deploy_drone::DeployDrone;
pub use one_insert::OneInsert;
pub use scooch_launch_and_landing::ScoochLaunchAndLanding;
pub use swap_trucks::SwapTrucks;
pub use two_opt::TwoOpt;
pub use three_opt::ThreeOpt;

pub trait Operator {
    fn generate_neighborhood(&self, solution: &crate::solution::Solution) -> Vec<crate::solution::Solution>;
    fn get_random_neighbor(&self, solution: &crate::solution::Solution, rng: &mut dyn rand::Rng) -> Option<crate::solution::Solution>;
}
