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
