#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TurnMode { Sequential, Simultaneous }

use common_shared::*;
use geometry::*;
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Action {
    Move { to: Cell },
    Moor { island: IslandId },
    AskClue,
    Bombard { target: ShipId },
    ProposeAlliance { to: ShipId },
    AcceptAlliance { proposer: ShipId },
    BreakAlliance,
    Declare,
    Hold,
}