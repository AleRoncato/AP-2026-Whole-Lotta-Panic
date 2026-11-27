use crate::brain::{Brain, Goal};
use crate::diplomacy::Diplomacy;
use crate::world::WorldModel;

/// Coordinates the Ship subsystems.
#[derive(Debug, Default)]
pub struct ShipAgent {
    world: WorldModel,
    diplomacy: Diplomacy,
    brain: Brain,
}

impl ShipAgent {
    /// Creates a new Ship.
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs the preliminary high-level decision cycle.
    ///
    /// This is not yet the Admiralty protocol interface.
    pub fn plan_round(&mut self, round: u32) -> Goal {
        // 1. Update knowledge
        self.world.update_round(round);

        // 2. Evaluate diplomatic situation
        let diplomatic_report = self.diplomacy.assess(&self.world);

        // 3. Select strategic objective
        self.brain.choose_goal(&self.world, &diplomatic_report)
    }
}
