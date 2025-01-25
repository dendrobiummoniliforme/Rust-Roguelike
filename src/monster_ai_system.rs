// use specs::{prelude::*, shred::PanicHandler};
// use crate::Name;

// use rltk::{Point, console};

// pub struct MonsterAI {}

// impl<'a> System<'a> for MonsterAI {
//     type SystemData = (
//         ReadExpect<'a, Point>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (player_pos) = data;

//         // Automatically move the player down the screen
//         let player_pos = *player_pos; // Access the Point value directly
//         player_pos.y += 1; // Adjust the value as needed for speed
//     }
// }