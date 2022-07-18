mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;
pub mod movement;

pub use crate::dungeon_crawler::prelude::*;
pub use collisions::*;
pub use end_turn::*;
pub use entity_render::*;
pub use map_render::*;
pub use player_input::*;
pub use random_move::*;
pub use movement::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build();
}

pub fn build_monster_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build();
}
