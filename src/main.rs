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
mod tetronimo;
pub use tetronimo::*;

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
        self.ecs.maintain(); // Apply changes to the world now.
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls(); // Clear the active terminal.
        
        let speed_mod = 2;
        self.tick_count += 1 * speed_mod;
        if self.tick_count % 10 == 0 {
            try_move_player(0, 1, &mut self.ecs);
        }

        self.runstate = player_input(self, ctx);
        // let tetrinonimo = spawn_tetris_block(&mut self.ecs, 20, 20, TetrisBlockType::L);
        // self.ecs.insert(tetrinonimo);

        draw_map(&self.ecs, ctx);
        render_tetronimo(&self.ecs, ctx);

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        //let map = self.ecs.fetch::<Map>();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(50, 50)?
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new(), runstate: RunState::Running, tick_count: 0 };
    gs.ecs.insert(context.clone());

    // Register components.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<TetrisBlock>();
 
    // Add map.
    let map: Map = Map::new_tetris_map();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    // Insert a point that follows the player around.
    // This is used to enable interaction with monsters.
    gs.ecs.insert(Point::new(player_x, player_y));

    let tetrinonimo = spawn_tetris_block(&mut gs.ecs, 20, 20, TetrisBlockType::L);
    gs.ecs.insert(tetrinonimo);

    rltk::main_loop(context, gs)
}

