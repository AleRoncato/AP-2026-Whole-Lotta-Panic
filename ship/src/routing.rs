//! Ship's routing algorithm.

use geometry::{Cell, Map, Terrain};
use shared::Round;
use std::collections::{HashMap, VecDeque};

#[expect(dead_code, reason = "Routing is not yet called")]
fn shortest_path(
    map: &impl Map,
    start: Cell,
    goals: &[Cell],
    start_round: Round,
    last_round: Round,
) -> Option<Vec<Cell>> {
    if !map.is_water(start) {
        return None;
    }

    let start_state = (start, start_round);
    let mut predecessors = HashMap::from([(start_state, start_state)]);
    let mut queue = VecDeque::from([start_state]);

    while let Some((cell, round)) = queue.pop_front() {
        if goals.contains(&cell) {
            let mut route = Vec::new();
            let mut current_state = (cell, round);

            while current_state != start_state {
                route.push(current_state.0);
                current_state = *predecessors.get(&current_state)?;
            }

            route.reverse();
            return Some(route);
        }

        if round > last_round {
            continue;
        }

        let Some(next_round) = round.checked_add(1) else {
            continue;
        };

        for neighbour in cell.neighbours() {
            if map.terrain(neighbour) == Some(Terrain::Clear) && !predecessors.contains_key(&(neighbour, next_round)) {
                predecessors.insert((neighbour, next_round), (cell, round));
                queue.push_back((neighbour, next_round));
            }
        }
    }

    None
}
