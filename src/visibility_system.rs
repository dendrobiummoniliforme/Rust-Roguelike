use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>, // If we do not have a map, this fails, hence Expect.
        Entities<'a>,
        WriteStorage<'a, Viewshed>, 
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (entity, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(pos.x, pos.y), 
                    viewshed.range, &*map
                );
                viewshed.visible_tiles.retain(
                    |p| p.x >= 0 && 
                    p.x < map.width && 
                    p.y >= 0 && 
                    p.y < map.height
                );

                let player_entity: Option<&Player> = player.get(entity);
                if player_entity.is_some() {
                    for tile in map.visible_tiles.iter_mut() {
                        *tile = false
                    };
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}