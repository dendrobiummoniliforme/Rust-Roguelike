use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map; // Tell this file that the module 'map' is located at ./
pub use map::*; // Import the map module for us in this file
mod player;
use player::*;
mod rect;
use rect::*;
mod visibility_system;
pub use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::*;
mod tetronimo_spawn_system;
pub use tetronimo_spawn_system::*;

struct State {
    pub ecs: World,
    pub runstate: RunState,
    pub tick_count: u32,
} // Braced struct declarations are not followed by a semi-colon.

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running
}

impl State {
    fn run_systems(&mut self) {
        //let mut vis = VisibilitySystem{};
        //let mut mob = MonsterAI{};
        let mut tetronimo = TetronimoSpawnSystem{};
        //vis.run_now(&self.ecs);
        //mob.run_now(&self.ecs);
        tetronimo.run_now(&self.ecs);
        self.ecs.maintain(); // Apply changes to the world now.
    }
}
const TETRIS_BLOCK: Tetronimo = Tetronimo {
    shape: [
        [Some(TileType::Floor), Some(TileType::Floor), None, None],
        [None, Some(TileType::Floor), None, None],
        [None, Some(TileType::Floor), None, None],
        [None, None, None, None],
    ],
    x: 5,
    y: 5
};

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls(); // Clear the active terminal.
        
        let speed_mod = 2;
        self.tick_count += 1 * speed_mod;
        if self.tick_count % 10 == 0 {
            try_move_player(0, 1, &mut self.ecs);
        }
        self.runstate = player_input(self, ctx);

        draw_map(&self.ecs, ctx);    // Assuming you have a mutable reference to the Rltk context

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        //let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(50, 50)?
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new(), runstate: RunState::Running, tick_count: 0 };

    // Register components.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();
 
    // Add map.
    let map: Map = Map::new_tetris_map();
    let (player_x, player_y) = map.rooms[0].center();

    for i in 0..4 { // Create a line of 4 entities
        gs.ecs.create_entity()
        // need a tetris block component
        // need to the attach a tetris block to a player by spawning it in
        // we can then manipulate the player's block while it is not at the bottom of the screen
        .with(Position { x: player_x + i, y: player_y }) // Adjust y position for each entity
        .with(Renderable {
            glyph: rltk::to_cp437('◘'),
            fg: RGB::named(rltk::VIOLET_RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Name { name: format!("Line {}", i + 1) }) // Unique name for each entity
        .build();
    }

    gs.ecs.insert(map);

    // Insert a point that follows the player around.
    // This is used to enable interaction with monsters.
    gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs)
}

