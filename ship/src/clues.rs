use std::collections::{BTreeMap, BTreeSet};
use std::collections::btree_map::Entry;
use geometry::*;
use common_clues::*;
use common_shared::{IslandId, ShipId};

pub(crate) struct ClueInfos {
    verified_clues: BTreeMap<IslandId, Clue>,
    candidates_cells: BTreeSet<Cell>,
    believed_candidates: BTreeSet<Cell>,
    unverified_clues: BTreeMap<ShipId,Vec<Clue>>,
    liars: BTreeSet<ShipId>,
    map: DefaultMap,
}

impl ClueInfos {
    pub(crate) fn new(default_map: DefaultMap) -> Self {
        let mut candidates_cells:BTreeSet<Cell> = BTreeSet::new();

        for c in default_map.clear_cells()  {
            //controlla sia adiacente a land.
            let mut valid = true;

            if !c.neighbours().iter().any(|&n| default_map.is_land(n)) {
                candidates_cells.insert(c);
            }

        }

        //controlliamo gli angoli
        for corner in default_map.corners() {
            candidates_cells.retain(|x| *x != corner && !corner.neighbours().contains(x));
        }

        ClueInfos {
            believed_candidates: candidates_cells.clone(),
            verified_clues: BTreeMap::new(),
            candidates_cells,
            unverified_clues: Default::default(),
            map: default_map,
            liars: BTreeSet::new(),
        }
    }

    pub(crate) fn add_clue(&mut self, clue: Clue, island: IslandId) {
        self.candidates_cells.retain(|&c| clue.holds(c));

        // teniamo quello piu forte
        match self.verified_clues.entry(island) {
            Entry::Vacant(slot) => {
                slot.insert(clue);
            }
            Entry::Occupied(mut slot) => {
                if slot.get().is_weaker_than(clue).unwrap_or(false) {
                    slot.insert(clue);
                }
            }
        }
    }

    pub(crate) fn add_clue_unverified(&mut self, clue: Clue, ship_id: ShipId) {
        //togliamo quelli sicuramente falsi
        let mut a = self.candidates_cells.clone();
        a.retain(|c| clue.holds(*c));
        //l'indizio era falso al 100%
        if a.is_empty() {
            self.liars.insert(ship_id);
        }

        match self.unverified_clues.get(&ship_id) {
            None => { self.unverified_clues.insert(ship_id, vec![clue]); },
            Some(v) => {
                let mut new = v.clone();
                new.push(clue);
                self.unverified_clues.insert(ship_id,new);
            }
        }

    }


    pub(crate) fn is_liar(&self, ship_id: ShipId) -> bool {self.liars.contains(&ship_id)}

}