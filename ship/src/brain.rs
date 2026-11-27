use crate::diplomacy::DiplomaticReport;
use crate::world::WorldModel;

/// High-level strategic objective.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Goal {
    /// Explore and collect information.
    #[default]
    Explore,

    /// Obtain additional clues.
    AcquireClue,

    /// Reach a candidate treasure location.
    ReachTreasure,

    /// Avoid an immediate threat.
    EvadeThreat,
}

#[derive(Debug, Default)]
pub(crate) struct Brain {
    current_goal: Goal,
    // TODO: finite_state_machine
    // TODO: utility_weights
    // TODO: strategic_memory
}

impl Brain {
    pub(crate) fn choose_goal(&mut self, _world: &WorldModel, report: &DiplomaticReport) -> Goal {
        // Temporary example:
        // immediate threats override the current goal.

        if report.danger {
            self.current_goal = Goal::EvadeThreat;
        }

        // TODO: evaluate candidate goals
        // TODO: compute utilities
        // TODO: update FSM

        self.current_goal
    }
}
