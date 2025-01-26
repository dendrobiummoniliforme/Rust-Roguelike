use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use crate::RunState;

use super::{Position, Player, Viewshed, State, Map};
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>(); // Feels odd to couple map to the player like this.

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

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
            return RunState::Paused 
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
                return RunState::Paused 
            }
        },
    }
    RunState::Running
}