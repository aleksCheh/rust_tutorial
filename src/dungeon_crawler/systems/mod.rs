mod chasing;
mod collisions;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod use_item;

pub use crate::dungeon_crawler::prelude::*;
pub use chasing::*;
pub use collisions::*;
pub use end_turn::*;
pub use entity_render::*;
pub use fov::*;
pub use hud::*;
pub use map_render::*;
pub use movement::*;
pub use player_input::*;
pub use random_move::*;
pub use tooltips::*;
pub use use_item::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(tooltips::tooltips_system())
        .add_system(hud::hud_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(use_item::use_item_system())
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build();
}

pub fn build_monster_scheduler() -> Schedule {
    return Schedule::builder()
        //.add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(use_item::use_item_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build();
}

#[allow(unused)]
pub fn build_dbg_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(tooltips::debug_ent_system())
        .build()
}
