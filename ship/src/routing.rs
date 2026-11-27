//! Ship's routing algorithm.

use geometry::{Cell, Map, Terrain};
use std::collections::{HashMap, VecDeque};

#[expect(dead_code, reason = "Routing is not yet called")]
fn shortest_path(map: &impl Map, start: Cell, goals: &[Cell]) -> Option<Vec<Cell>> {
    if !map.is_water(start) {
        return None;
    }

    let mut predecessors = HashMap::from([(start, start)]);
    let mut queue = VecDeque::from([start]);

    while let Some(curr_cell) = queue.pop_front() {
        if goals.contains(&curr_cell) {
            let mut route = Vec::new();
            let mut current = curr_cell;

            while current != start {
                route.push(current);
                current = *predecessors.get(&current)?;
            }

            route.reverse();
            return Some(route);
        }

        for neighbour in curr_cell.neighbours() {
            if map.terrain(neighbour) == Some(Terrain::Clear) && !predecessors.contains_key(&neighbour) {
                predecessors.insert(neighbour, curr_cell);
                queue.push_back(neighbour);
            }
        }
    }

    None
}
