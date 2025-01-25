use specs::prelude::*;
use crate::{Map, RunState, State, TileType};

use super::{Position, Player, Renderable, TetrisBlock, Name};
use rltk::{Rltk, VirtualKeyCode, RGB};

// The type of Tetris piece
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TetrisBlockType {
    I, O, T, S, Z, J, L
}

pub enum TetronimoTypeType {
    active,
    inactive
}

pub fn get_tetris_offsets(block_type: TetrisBlockType) -> Vec<(i32, i32)> {
    match block_type {
        TetrisBlockType::I => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        TetrisBlockType::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        TetrisBlockType::T => vec![(0, 0), (1, 0), (2, 0), (1, 1)],
        TetrisBlockType::S => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
        TetrisBlockType::Z => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
        TetrisBlockType::J => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
        TetrisBlockType::L => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
    }
}

/// Spawns a single "parent" entity that knows its type of Tetris block.
/// We can manipulate `Position` for movement, and the system will handle
/// drawing the offsets in a custom render step.
pub fn spawn_tetris_block(ecs: &mut World, x: i32, y: i32, block_type: TetrisBlockType) -> Entity {
    // Retrieve the shape offsets
    let offsets = get_tetris_offsets(block_type);

    // Create the new TetrisBlock entity
    let entity = ecs.create_entity()
    .with(Position { x, y })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(TetrisBlock {
        block_type,
        offsets,
        tile: TetronimoTypeType::active
    })
    .with(Player {})
    .with(Name { name: format!("{:?} Block", block_type) })
    .build();

    entity
}

pub fn render_tetronimo(world: &World, ctx: &mut Rltk) {
    let entities = world.entities();
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let tetronimos = world.read_storage::<TetrisBlock>();

    for (entity, pos, render) in (&entities, &positions, &renderables).join() {
        if let Some(block) = tetronimos.get(entity) {
            for (dx, dy) in &block.offsets {
                ctx.set(pos.x + dx, pos.y +dy, render.fg, render.bg, render.glyph);
            }
        } else {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl TetrisBlock {
    /// Get the idx for an xy coordinate.
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        // We use usize such that we do not
        // return a negative idx.
        // As usize is unsigned.
        (y as usize * 80) + (x as usize)
    } 
}