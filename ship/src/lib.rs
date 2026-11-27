//! Autonomous Ship AI.

mod agent;
mod brain;
mod diplomacy;
pub mod routing;
mod world;

mod clues;

pub use agent::ShipAgent;
pub use brain::Goal;