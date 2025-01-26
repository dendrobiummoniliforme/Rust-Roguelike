use rltk::{ RGB, Rltk,};
use specs::prelude::*;
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
}