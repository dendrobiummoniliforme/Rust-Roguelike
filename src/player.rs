use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use crate::{tetronimo, RunState, TetrisBlock, TetronimoTypeType};

use super::{Position, Player, Viewshed, TileType, State, Map};
use std::cmp::{min, max};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let map = ecs.fetch::<Map>();
    let mut positions = ecs.write_storage::<Position>();
    let mut blocks = ecs.write_storage::<TetrisBlock>();

    for (pos, block) in (&mut positions, &mut blocks).join() {
        // Check if ANY of the squares would collide
        let mut blocked = false;
        for (ox, oy) in &block.offsets {
            let new_x = pos.x + ox + dx;
            let new_y = pos.y + oy + dy;
            let idx = map.xy_idx(new_x, new_y);
            if map.tiles[idx] == TileType::Wall {
                blocked = true;
                break;
            }
        }

        // If not blocked, move the whole piece
        if !blocked {
            pos.x += dx;
            pos.y += dy;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { 
            return RunState::Paused 
        }
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            // VirtualKeyCode::Up |
            // VirtualKeyCode::Numpad8 |
            // VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::A => rotate_player(&mut gs.ecs),

            _ => { 
                return RunState::Paused 
            }
        },
    }
    RunState::Running
}

fn rotate_player(ecs: &mut World) {
    let mut tetronimo = ecs.write_storage::<TetrisBlock>();
    let mut positions = ecs.write_storage::<Position>();
    let map = ecs.fetch::<Map>();

    for (pos, tetronimo) in (&mut positions, &mut tetronimo).join() {
        let mut blocked = false;
        for (ox, oy) in tetronimo.offsets.iter_mut() {
            let new_x = pos.x + oy.clone();
            let new_y = pos.y - (ox.clone());
            // let new_x = pos.x + ox + dx;
            // let new_y = pos.y + oy + dy;
            let idx = map.xy_idx(new_x, new_y);
            //let idx_tetris = tetronimo.xy_idx(new_x, new_y);
            if map.tiles[idx] == TileType::Wall {
                blocked = true;
                break;
            }
        } 
        for (ox, oy) in tetronimo.offsets.iter_mut() {
            if !blocked {
                let old_x = *ox;
                *ox = *oy;
                *oy = -old_x;
            }
        }
    }
}