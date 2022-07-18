mod entity_render;
mod map_render;
mod player_input;
mod collisions;
mod random_move;

pub use crate::dungeon_crawler::prelude::*;
pub use entity_render::*;
pub use map_render::*;
pub use player_input::*;
pub use collisions::*;
pub use random_move::*;

pub fn build_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build();
}
