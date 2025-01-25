use rltk::RGB;

use specs::prelude::*;
use super::{Position, Player, Tetronimo};

pub struct TetronimoSpawnSystem {}

// I can move this to a System that draws a Tetronimo
// this is how we can turn this into a component.
// draw --> run and we can set a "isSpawned = true" and likey an "isInPlay = true"
impl<'a> System<'a> for TetronimoSpawnSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Tetronimo>,
    );

     fn run(&mut self, data: Self::SystemData) {
        let (mut pos, player, mut tetronimo) = data;

        for (pos, _player, tetronimo) in (&pos, &player, &tetronimo).join() {
            for (row_index, row) in tetronimo.shape.iter().enumerate() {
                for (col_index, &tile) in row.iter().enumerate() {
                    if let Some(tile_type) = tile {
                        let glyph = rltk::to_cp437('*');
                        ctx.set(
                            pos.x + col_index as i32,
                            pos.y + row_index as i32,
                            RGB::from_f32(0.5, 0.5, 0.5),
                            RGB::from_f32(0.0, 0.0, 0.5),
                            glyph
                        )
                    }
                }
            }
    }
}
}