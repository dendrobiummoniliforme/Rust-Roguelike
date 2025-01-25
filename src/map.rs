use rltk::{Point, BaseMap, Algorithm2D, RandomNumberGenerator, Rltk, RGB};
use super::{Rect, World};
use std::cmp::{max, min};

// Adding PartialEq lets us compare two tile types to see if they match
// that is, tile1 == tile2. I assume this means that equality on objects
// does not normally do type matching but exact matching, so an instance
// of one object is not normally equally to an instance of another.
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    /// Get the idx for an xy coordinate.
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        // We use usize such that we do not
        // return a negative idx.
        // As usize is unsigned.
        (y as usize * 80) + (x as usize)
    }  

    /// Given a Room and a slice of Map apply the Room to the Map by
    /// mutating it.
    pub fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Makes a map with he tetris specifications.
    pub fn new_tetris_map() -> Map {
        let mut map = Map{
            tiles : vec![TileType::Wall; 80*50],
            rooms : Vec::new(),
            width : 80,
            height: 50,
            revealed_tiles : vec![false; 80*50],
            visible_tiles : vec![false; 80*50]
        };

        let new_room = Rect::new(19, 4, 9, 39);
        map.apply_room_to_map(&new_room);
        map.rooms.push(new_room);
        map
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

/// Given a slice of Map, and the ctx
/// Apply a color and symbol to each idx based on the
/// TileType.
pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (_idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon tile type
        let glyph;
        let fg;
        
        match tile {
            TileType::Floor => {
                fg = RGB::from_f32(0.0, 0.5, 0.5);
                glyph = rltk::to_cp437('.');
            } 
            TileType::Wall => {
                fg = RGB::named(rltk::SNOW3);
                glyph = rltk::to_cp437('☼');
            }
        }
        ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}