use rltk::{Rltk, RGB, RandomNumberGenerator};
use super::Rect;
use std::{char::MAX, cmp::{max, min}};

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
    pub height: i32
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
    pub fn apply_room_to_map(&mut self, room: &Rect, map: &mut [TileType]) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 ..= room.x2 {
                map[self.xy_idx(x, y)] = TileType::Floor;
            }
        }
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarentees that
    /// it won't look awful.
    pub fn new_map_test(&mut self) -> Vec<TileType> {
        let mut map = vec![TileType::Floor; 80*50];
        
        for x in 0..80 {
            map[self.xy_idx(x, 0)] = TileType::Wall;
            map[self.xy_idx(x, 49)] = TileType::Wall;
        }

        for y in 0..50 {
            map[self.xy_idx(0, y)] = TileType::Wall;
            map[self.xy_idx(79, y)] = TileType::Wall;
        }

        // Now we will randomly splat a bunch of walls. It won't be pretty, 
        // but it's a decent illustration.
        // First, obtain te thread-local RNG:
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = self.xy_idx(x, y);

            if idx != self.xy_idx(40, 25) {
                map[idx] = TileType::Wall;
            }
        }

        map
    }

    /// Makes a map with two rooms connected by corridors.
    /// Returns a list of rooms.
    pub fn new_map_rooms_and_corridors(&mut self) -> (Vec<Rect>, Vec<TileType>) {
        let mut map = vec![TileType::Wall; 80*50];
        
        let mut rooms: Vec<Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;

            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                self.apply_room_to_map(&new_room, &mut map);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }
                rooms.push(new_room);
            }
        }

        (rooms, map)
    }  

    fn apply_horizontal_tunnel(&mut self, map: &mut [TileType], x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                map[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, map: &mut [TileType], y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                map[idx as usize] = TileType::Floor;
            }
        }
    }
}

/// Given a slice of Map, and the ctx
/// Apply a color and symbol to each idx based on the
/// TileType.
pub fn draw_map(&mut self, map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        // Render a tile depending upon tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}