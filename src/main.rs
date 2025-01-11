use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

struct State {
    ecs: World
} // Braced struct declarations are not followed by a semi-colon.

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}

#[derive(Component, Debug)]
struct Player {}

// Adding PartialEq lets us compare two tile types to see if they match
// that is, tile1 == tile2. I assume this means that equality on objects
// does not normally do type matching but exact matching, so an instance
// of one object is not normally equally to an instance of another.
#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}
/// Get the idx for an xy coordinate.
fn xy_idx(x: i32, y: i32) -> usize {
    // We use usize such that we do not
    // return a negative idx.
    // As usize is unsigned.
    (y as usize * 80) + (x as usize)
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        // If we are not on a wall.
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(79, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode:: Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode:: Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode:: Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode:: Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

impl<'a> System<'a> for LeftWalker {
    // Tell the system what data it needs access to, and what it needs to be able to
    // do with that data.
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    // This is similar to how we rendered code.
    // But it is different.
    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain(); // Apply changes to the world now.
    }
}

impl GameState for State {
    // For the Struct State, implement the Tick function from
    // the trait GameState.
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls(); // Clear the active terminal.

        // Update before rendering.
        self.run_systems();

        // Get input for Player before rendering.
        player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();

        draw_map(&map, ctx);
    
        // Join these two components.
        // Literally a union.
        // It's implicit Union, but it works, as each Entity already has a
        // unique id tied to it from the build step.
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{ ecs: World::new() };

    gs.ecs.insert(new_map());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // Player Entity.
    gs.ecs.create_entity()
    .with(Position { x: 40, y: 25 })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(Player {})
    .build();

    
    for i in 0..10 {
        gs.ecs.create_entity()
        .with(Position { x: i * 7, y: 20 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(LeftMover {})
        .build();
    }

    rltk::main_loop(context, gs)
}

