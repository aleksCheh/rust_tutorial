mod camera;
mod components;
mod crawler_map;
mod crawler_map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::Algorithm2D;
    pub use bracket_lib::prelude::*;
    pub use std::io::{stdin, Read};
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::dungeon_crawler::camera::*;
    pub use crate::dungeon_crawler::components::*;
    pub use crate::dungeon_crawler::crawler_map::*;
    pub use crate::dungeon_crawler::crawler_map_builder::*;
    pub use crate::dungeon_crawler::spawner::*;
    pub use crate::dungeon_crawler::systems::*;
    pub use crate::dungeon_crawler::turn_state::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

//use std::thread::spawn;

use self::prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    //debug_systems: Schedule,
    // map: CrawlerMap,
    // player: Player,
    // camera: Camera,
}
impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        #[allow(non_snake_case)]
        let crawlerMapBuilder = CrawlerMapBuilder::new(&mut rng);
        spawn_player(&mut ecs, crawlerMapBuilder.player_start);
        spawn_amulet(&mut ecs, crawlerMapBuilder.amulet_start);
        crawlerMapBuilder
            .monster_spawn
            .iter()
            .for_each(|pos| spawn_entity(&mut ecs, &mut rng, *pos));
        println!("Start inserting recources");
        resources.insert(crawlerMapBuilder.map);
        resources.insert(Camera::new(crawlerMapBuilder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(crawlerMapBuilder.theme);

        State {
            ecs: ecs,
            resources: resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            //debug_systems: build_dbg_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_centered(23, "The End was always near");
        ctx.print_centered(25, "Press Space to return");

        let key = match ctx.key {
            Some(it) => it,
            _ => return,
        };
        println!("Before match");
        match key {
            VirtualKeyCode::Space => self.reset_state(),
            _ => println!("another key"),
        }
    }
    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_centered(23, "You found it!");
        ctx.print_centered(25, "Press Space to triumph");

        let key = match ctx.key {
            Some(it) => it,
            _ => return,
        };

        match key {
            VirtualKeyCode::Space => self.reset_state(),
            _ => println!("another key"),
        }
    }

    fn reset_state(&mut self) {
        println!("Space");
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        #[allow(non_snake_case)]
        let crawlerMapBuilder = CrawlerMapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, crawlerMapBuilder.player_start);
        spawn_amulet(&mut self.ecs, crawlerMapBuilder.amulet_start);
        crawlerMapBuilder
            .monster_spawn
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));
        self.resources.insert(crawlerMapBuilder.map);
        self.resources
            .insert(Camera::new(crawlerMapBuilder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(crawlerMapBuilder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        // self.debug_systems.execute(&mut self.ecs, &mut self.resources);
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
        }
        render_draw_buffer(ctx).expect("Render Error!");
    }
}

pub fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_resource_path("resources/")
        .with_tile_dimensions(32, 32)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
mod test {
    #[test]
    fn run() {
        println!(
            "Max f32 value as value: {} and with & {}",
            f32::MAX,
            &f32::MAX
        );
        super::main().unwrap()
    }
}
