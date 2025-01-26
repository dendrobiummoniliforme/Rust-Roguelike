use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{Position, Player, Viewshed, State, Map, CombatStats, RunState, WantsToMelee};
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let map = ecs.fetch::<Map>(); // Feels odd to couple map to the player like this.

    for (entity, _player, pos, viewshed) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
                if let Some(_target) = target {
                    // If we have a target lets attempt to melee it
                    wants_to_melee.insert(
                        entity, // Attacker
                        WantsToMelee{ target: *potential_target }
                    ).expect("Add target failed");
                    console::log(&format!("From Hell's Heart, I stab thee!"));
                    return; // So we do not move after attacking
            }
        }

        // If we are not on blocked.
        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(79, max(0, pos.y + delta_y));

            // Update the point's position.
            // This is a point that is following the player's position.
            let mut point_pos = ecs.write_resource::<Point>();
            point_pos.x = pos.x;
            point_pos.y = pos.y;
            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { 
            return RunState::AwaitingInput
        }
        Some(key) => match key {
            // LEFT
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            // RIGHT
            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            // UP
            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            // DOWN
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            // UP RIGHT
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

            // UP LEFT
            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

            // DOWN RIGHT
            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            // DOWN LEFT
            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            _ => { 
                return RunState::AwaitingInput
            }
        },
    }
    RunState::PlayerTurn
}