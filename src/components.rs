use bevy::{
    math::Vec2,
    prelude::{Component, KeyCode},
};

pub enum PlayerNumber {
    Player1,
    Player2,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub axis_xy: (bool, bool),
    pub bounces: bool,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Player {
    pub player: PlayerNumber,
    pub player_uo_down_keys: (KeyCode, KeyCode),
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Stats {
    pub player1: u32,
    pub player2: u32,
}
