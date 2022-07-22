use ball::BallPlugin;
use bevy::prelude::*;
use components::{Ball, Movable, SpriteSize, Velocity};
use player::PlayerPlugin;
mod ball;
pub mod components;
mod player;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const PLAYER_CUSTOM_SIZE: (f32, f32) = (50., 200.);
pub const BALL_SIZE: (f32, f32) = (10., 10.);
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

//resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

fn setup_system(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(setup_system)
        .add_system(movement_system)
        .run();
}

fn movement_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(&mut Velocity, &mut Transform, &Movable)>,
) {
    for (mut velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        let x = translation.x + velocity.x * TIME_STEP * BASE_SPEED;
        let y = translation.y + velocity.y * TIME_STEP * BASE_SPEED;

        translation.x = x;
        translation.y = y;
    }
}

// fn paddle_bounce(ball_query: Query<(&Transform, &mut Velocity, &SpriteSize), With<Ball>>, player_query: Query<>) {}
