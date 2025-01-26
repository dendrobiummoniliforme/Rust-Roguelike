use specs::prelude::*;
use super::{Map, Position, BlocksTile};

pub struct MapIndexingSystem {}

impl <'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;

        // Initial population of blocked tiled.
        map.populate_blocked();

        // Then, for all Entities with BlocksTile Component,
        // update the tile on the map to blocked.
        for (position, _blocks) in (&position, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked[idx] = true;
        }
    }
}