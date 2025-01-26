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
mod map_indexing_system;
pub use map_indexing_system::*;
mod melee_combat_system;
pub use melee_combat_system::*;
mod damage_system;
pub use damage_system::*;

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
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
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
 
    // Add map.
    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // Player Entity.
    let player_entity = gs.ecs.create_entity()
    .with(Position { x: player_x, y: player_y })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(Player {})
    .with(Name { name: "Player".to_string() })
    .with(Viewshed { visible_tiles: Vec::new(), dirty: true, range: 8 })
    .with(CombatStats { max_hp: 30, hp: 30, defense: 2, power: 5 })
    .build();
    
    // Create and render Monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() { // Skip the player's room
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('g'); name = "Goblin".to_string(); }
            _ => { glyph = rltk::to_cp437('o'); name = "Orc".to_string(); }
        }

        gs.ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Monster {})
        .with(Name { name: format!("{} #{}", &name, i) })
        .with(BlocksTile {})
        .with(CombatStats { max_hp: 16, hp: 16, defense: 1, power: 4 })
        .build();
    }

    gs.ecs.insert(map);

    // Insert a point that follows the player around.
    // This is used to enable interaction with monsters.
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);

    rltk::main_loop(context, gs)
}

