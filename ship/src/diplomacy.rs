use crate::world::WorldModel;

#[derive(Debug, Default)]
pub(crate) struct DiplomaticReport {
    pub(crate) danger: bool,
    // TODO: opponent_assessments
    // TODO: suggested_alliance
    // TODO: suggested_messages
    // TODO: policy_constraints
}

#[derive(Debug, Default)]
pub(crate) struct Diplomacy;

impl Diplomacy {
    pub(crate) fn assess(&self, _world: &WorldModel) -> DiplomaticReport {
        // TODO: implement diplomatic policies

        DiplomaticReport::default()
    }
}
