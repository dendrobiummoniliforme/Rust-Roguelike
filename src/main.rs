use gamelog::GameLog;
use rltk::{GameState, Point, Rltk, Sprite, SpriteSheet, RGB};
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
mod spawner;
pub use spawner::*;
mod map_indexing_system;
pub use map_indexing_system::*;
mod melee_combat_system;
pub use melee_combat_system::*;
mod damage_system;
pub use damage_system::*;
mod gui;
pub use gui::*;
mod gamelog;

struct State {
    pub ecs: World,
} // Braced struct declarations are not followed by a semi-colon.

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);

        let mut map_index = MapIndexingSystem{};
        map_index.run_now(&self.ecs);

        let mut melee_combat = MeleeCombatSystem{};
        melee_combat.run_now(&self.ecs);

        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
        
        self.ecs.maintain(); // Apply changes to the world now.
    }
}

impl GameState for State {
    // For the Struct State, implement the Tick function from
    // the trait GameState.
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls(); // Clear the active terminal.

        let mut new_runstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            new_runstate = *runstate;
        }
        match new_runstate {
            RunState::PreRun => {
                self.run_systems();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_runstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_runstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_runstate = RunState::AwaitingInput;
            }
        }
        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_runstate;
        }

        damage_system::delete_the_dead(&mut self.ecs);
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
    
        // Join these two components.
        // Literally a union.
        // It's implicit Union, but it works, as each Entity already has a
        // unique id tied to it from the build step.
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        gui::draw_ui(&self.ecs, ctx);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fullscreen(true)
        .build()?;
    context.with_post_scanlines(true);
    context.screen_burn_color(RGB::named(rltk::MAGENTA));
    let mut gs = State { 
        ecs: World::new(),
    };

    // Register components.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();

    gs.ecs.insert(rltk::RandomNumberGenerator::new());
 
    // Add map.
    let map: Map = Map::new_map_rooms_and_corridors();
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);
    
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // Insert a point that follows the player around.
    // This is used to enable interaction with monsters.
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog { entries: vec!["Welcome to Rusty Roguelike".to_string()] });

    rltk::main_loop(context, gs)
}

