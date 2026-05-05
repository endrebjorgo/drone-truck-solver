pub mod one_insert;
pub mod deploy_drone;
pub mod scooch_launch_and_landing;
pub mod swap_trucks;

pub use one_insert::OneInsert;
pub use deploy_drone::DeployDrone;
pub use scooch_launch_and_landing::ScoochLaunchAndLanding;
pub use swap_trucks::SwapTrucks;

pub trait Operator {
    fn generate_neighborhood(&self, solution: &crate::solution::Solution) -> Vec<crate::solution::Solution>;
    fn get_random_neighbor(&self, solution: &crate::solution::Solution, rng: &mut dyn rand::Rng) -> Option<crate::solution::Solution>;
}
