pub use crate::dungeon_crawler::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Player;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Enemy;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MovingRandomly;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
