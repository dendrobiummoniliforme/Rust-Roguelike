use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

use super::{Viewshed, Monster, Name, Position, Renderable, Player, CombatStats, BlocksTile, Rect, MAPWIDTH};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

/// Create a Player
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
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
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;

    // Scope the roll and rng to free the borrow quicker for rng.
    {
        let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }

    match roll {
        1 => { orc(ecs, x, y) }
        _ => { goblin(ecs, x, y) }
    }
}
// Specific Monsters
fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc");
}

fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin");
}

/// Create a Monster
fn monster<S: ToString>(ecs: &mut World, x: i32, y:i32, glyph: rltk::FontCharType, name: S) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Monster {})
        .with(Name { name: name.to_string() })
        .with(BlocksTile {})
        .with(CombatStats { max_hp: 16, hp: 16, defense: 1, power: 4 })
        .build();
}

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    
    let num_monsters = ecs.write_resource::<RandomNumberGenerator>()
        .roll_dice(1, MAX_MONSTERS + 2) - 3;

    // Generate spawn points
    for _i in 0 .. num_monsters {
        let mut added = false;
        while !added {
            /*
            Notes to myself the original version used a scope to contain the rng borrow.
            Can achieve the same thing with a short borrow.

            The new version is less efficient. We make two access calls to rng in favor of
            not using the scope functionality.

            Since I am new to Rust, I actually have no idea what is better.

            I think the rng needs internal state, which is why roll_dice needs mutable access
            to the RandomNumberGenerator instance. So that it can maintain a random roll between
            each call (not giving the same number).

            // ORIGINAL VERSION - Long-lived borrow
            let mut rng = ecs.write_resource::<RandomNumberGenerator>();  // Start borrow
            let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
            // rng borrow is still active here, preventing other ecs uses

            // FIXED VERSION - Short-lived borrows
            let x = ecs.write_resource::<RandomNumberGenerator>()  // Start borrow
                .roll_dice(1, i32::abs(room.x2 - room.x1)) as usize;    // End borrow
            let y = ecs.write_resource::<RandomNumberGenerator>()  // New borrow
                .roll_dice(1, i32::abs(room.y2 - room.y1)) as usize;    // End borrow
             */
            let mut rng = ecs.write_resource::<RandomNumberGenerator>();
            let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
            let idx = (y * MAPWIDTH) + x;
            if !monster_spawn_points.contains(&idx) {
                monster_spawn_points.push(idx);
                added = true;
            }
        }
    }

    // Actually spawn the monsters
    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }
}