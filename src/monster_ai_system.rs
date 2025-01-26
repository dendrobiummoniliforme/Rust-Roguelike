use specs::prelude::*;

use super::{Viewshed, Monster, Map, Name, Position};
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map, 
            player_pos, 
            mut viewshed, 
            monster, 
            name, 
            mut monster_pos
        ) = data;

        for (viewshed, _monster, name, monster_pos) in (&mut viewshed, &monster, &name, &mut monster_pos).join() {
            // In Rust, ReadExpect<'a, T> (from specs) is essentially a smart pointer (it implements Deref<Target = T>). 
            // This means player_pos is not itself a Point, but rather a wrapper that can be dereferenced to a Point.
            // *player_pos uses the Deref implementation to get the underlying Point value.
            // Then &*player_pos takes a reference to that Point.
            // 1. for each tick we check to see if a player is in the monster's view
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(
                    Point::new(monster_pos.x, monster_pos.y), 
                    *player_pos
                );

                if distance < 1.5 {
                    // Helper log function that outputs correctly to a browser or terminal
                    // depending on the environment.
                    console::log(&format!("{} shouts insults", name.name));
                    return;
                }

                // 2. if it is we perform an A* Search to find a path to the player
                let path = rltk::a_star_search(
                    map.xy_idx(monster_pos.x, monster_pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map
                );

                console::log(path.success);

                // 3. if we are able to find a path
                if path.success && path.steps.len() > 1 {
                    // 4. we then update the monster's position by moving it along the path by 1
                    monster_pos.x = path.steps[1] as i32 % map.width;
                    monster_pos.y = path.steps[1] as i32 / map.width;
                    
                    // 5. and update the viewshed's current status (to allow it to check if we are still in range)
                    viewshed.dirty = true;
                }
            }
        }
    }
}