use specs::{prelude::*, rayon::string};
use specs_derive::*;
use rltk::RGB;

use crate::{TetronimoTypeType, TileType};

use super::TetrisBlockType;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

/// Viewshed means "what can I see from here?"
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub dirty: bool,
    pub range: i32,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}

// Holds the shape definition of a Tetris block.
// Typically, each block has up to four squares offset from its origin.
#[derive(Component)]
pub struct TetrisBlock {
    pub block_type: TetrisBlockType,
    // Relative coordinates for each of the 4 squares in the shape
    // E.g. For the “I” block horizontally: [ (0,0), (1,0), (2,0), (3,0) ]
    pub offsets: Vec<(i32, i32)>,
    pub tile: TetronimoTypeType
}