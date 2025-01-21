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
                // Clear the tiles in the visibility data store.
                // Reset the flag for the entity.
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(pos.x, pos.y), 
                    viewshed.range, &*map
                );

                // Draw the FOV for the entity given it's range
                // and the slice of map their are anchored to.
                // This says that for these constraints
                // return all tiles that are inside the map
                // and tell the entity to retain these.
                // This then prevents the entity from
                // drawing or seeing through walls.
                viewshed.visible_tiles.retain(
                    |point| 
                    point.x >= 0 && 
                    point.x < map.width && 
                    point.y >= 0 && 
                    point.y < map.height
                );


                // The interplay between Map's visible and revealed tiles is how
                // we can maintain a trinary state for the rendering of these tiles.
                //   revealed === visbile === false is fully dark
                //   revealed === visible === true is fully colored
                //   revealed === true && visible === false is greyscale.
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