mod player_input;
mod map_render;
pub use crate::dungeon_crawler::prelude::*;
pub use map_render::*;
pub use player_input::*;
pub fn build_scheduler() -> Schedule {
    return Schedule::builder().add_system(player_input::player_input_system())
    .add_system(map_render::map_render_system())
    .build();
}
