use specs::{prelude::*, shred::PanicHandler};
use crate::Name;

use super::{Viewshed, Monster};
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            // In Rust, ReadExpect<'a, T> (from specs) is essentially a smart pointer (it implements Deref<Target = T>). 
            // This means player_pos is not itself a Point, but rather a wrapper that can be dereferenced to a Point.
            // *player_pos uses the Deref implementation to get the underlying Point value.
            // Then &*player_pos takes a reference to that Point.
            if viewshed.visible_tiles.contains(&*player_pos) {
                // Helper log function that outputs correctly to a browser or terminal
                // depending on the environment.
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}