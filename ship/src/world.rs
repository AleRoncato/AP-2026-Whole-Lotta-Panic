#[derive(Debug, Default)]
pub(crate) struct WorldModel {
    round: u32,
    // TODO: map
    // TODO: own_ship_state
    // TODO: verified_clues
    // TODO: unverified_clues
    // TODO: candidate_cells
    // TODO: observed_opponents
}

impl WorldModel {
    pub(crate) fn update_round(&mut self, round: u32) {
        self.round = round;
    }
}
