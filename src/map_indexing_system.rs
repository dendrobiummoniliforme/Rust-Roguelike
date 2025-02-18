use specs::prelude::*;
use super::{Map, Position, BlocksTile};

pub struct MapIndexingSystem {}

impl <'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        // Initial population of blocked tiled.
        map.populate_blocked();

        // Clear all entities that are in the maps list of known
        // tile content.
        map.clear_content_index();
        
        for (position, entity) in (&position, &entities).join() {
            let idx = map.xy_idx(position.x, position.y);

            // Check to see if an entity has a BlocksTile component.
            // if it does, update the maps blocked list.
            let associated_blockers: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_) = associated_blockers {
                map.blocked[idx] = true;
            }

            // Push the entity to the appropriate index slot. It's a copy
            // type, so we don't need to clone it (we want to avoid moving it out of the ECS!)
            map.tile_content[idx].push(entity);
        }
    }
}