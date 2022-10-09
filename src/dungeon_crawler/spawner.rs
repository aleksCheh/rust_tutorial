pub use crate::dungeon_crawler::prelude::*;

pub fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}
pub fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
//pub struct Spawner {}
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            max: 20,
            current: 20,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: glyph,
        },
        MovingRandomly,
        Health {
            max: hp,
            current: hp,
        },
        Name(name),
        ChasingPlayer,
        FieldOfView::new(6),
    ));
}

pub fn spawn_amulet(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_healing(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        Name("Health Poition".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_map(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap,
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    match rng.roll_dice(1, 6) {
        1 => spawn_healing(ecs, pos),
        2 => spawn_map(ecs, pos),
        _ => spawn_enemy(ecs, rng, pos),
    }
}
