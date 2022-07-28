use crate::dungeon_crawler::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = match health_query.iter(ecs).nth(0) {
        Some(t) => t,
        None => &Health {
            current: 20,
            max: 20,
        },
    };
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(ORANGE_RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health {} / {}", player_health.current, player_health.max),
        ColorPair::new(ANTIQUEWHITE2, RED),
    );
    draw_batch.submit(10_000).expect("Batch hud error");
}
