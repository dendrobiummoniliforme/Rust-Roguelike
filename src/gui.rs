use rltk::{ Point, Rltk, RGB};
use specs::prelude::*;
use crate::{Map, Name, Position};

use super::{CombatStats, Player, GameLog};

const GUI_HEIGHT: usize = 43;
const GUI_WIDTH: usize = 79;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0, 
        GUI_HEIGHT, 
        GUI_WIDTH, 
        6, 
        RGB::named(rltk::WHITE), 
        RGB::named(rltk::BLACK)
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let game_log = ecs.fetch::<GameLog>();

    let mut y = 44;
    for s in game_log.entries.iter().rev() {
        if y < 49 { 
            ctx.print(2, y, s) 
        }
        y += 1;
    }

    for (_player, combat_stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {}", combat_stats.hp, combat_stats.max_hp);
        ctx.print_color(
            12, 
            GUI_HEIGHT, 
            RGB::named(rltk::YELLOW), 
            RGB::named(rltk::BLACK), 
            &health
        );
        ctx.draw_bar_horizontal(
            28, 
            GUI_HEIGHT, 
            51, 
            combat_stats.hp, 
            combat_stats.max_hp, 
            RGB::named(rltk::RED), 
            RGB::named(rltk::BLACK)
        );
    }

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    // TODO I need to figure out how this actually works.
    // I wrote it out but I need to process the way this actually
    // draws the text.
    // What I do know is that at certain points (near an edge of a map)
    // it will draw to the left or right of the entity you are drawing a tooltip for.
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height {
        return;
    }

    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for str in tooltip.iter() {
            if width < str.len() as i32 {
                width = str.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;

            for str in tooltip.iter() {
                ctx.print_color(
                    left_x, 
                    y, 
                    RGB::named(rltk::WHITE), 
                    RGB::named(rltk::GREY), 
                    str
                );

                let padding = (width - str.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i, 
                        y, 
                        RGB::named(rltk::WHITE), 
                        RGB::named(rltk::GREY), 
                        &" ".to_string()
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x, 
                arrow_pos.y, 
                RGB::named(rltk::WHITE), 
                RGB::named(rltk::GREY), 
                &"<-".to_string()
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for str in tooltip.iter() {
                ctx.print_color(
                    left_x + 1, 
                    y,
                    RGB::named(rltk::WHITE), 
                    RGB::named(rltk::GREY), 
                    str
                );
                let padding = (width - str.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i, 
                        y, 
                        RGB::named(rltk::WHITE), 
                        RGB::named(rltk::GREY), 
                        &" ".to_string()
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x, 
                arrow_pos.y, 
                RGB::named(rltk::WHITE), 
                RGB::named(rltk::GREY), 
                &"->".to_string()
            );
        }
    }
}