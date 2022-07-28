use crate::dungeon_crawler::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();

    let player_fov = <&FieldOfView>::query()
    .filter(component::<Player>())
    .iter(ecs)
    .nth(0)
    .unwrap();

    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| map_pos == **pos && player_fov.visible_tiles.contains(*pos))
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, display);
        });
    draw_batch.submit(10100).expect("Batch Tooltip Error");
}

#[system]
#[read_component(Point)]
#[allow(unused_variables)]
pub fn debug_ent(sub_world: &SubWorld) {
    let mut i = 0;
    let mut ent = <(Entity, &Point)>::query();
    ent.iter(sub_world).for_each(|(e, p)| {
        i += 1;
        println!("counter: {}", i);
    });
}
