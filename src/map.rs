use rltk::{Rltk, RGB};

// Adding PartialEq lets us compare two tile types to see if they match
// that is, tile1 == tile2. I assume this means that equality on objects
// does not normally do type matching but exact matching, so an instance
// of one object is not normally equally to an instance of another.
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

/// Get the idx for an xy coordinate.
pub fn xy_idx(x: i32, y: i32) -> usize {
    // We use usize such that we do not
    // return a negative idx.
    // As usize is unsigned.
    (y as usize * 80) + (x as usize)
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarentees that
/// it won't look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];
    
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we will randomly splat a bunch of walls. It won't be pretty, 
    // but it's a decent illustration.
    // First, obtain te thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);

        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

/// Draw a map using a 
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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