use ball::BallPlugin;
use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use components::{Ball, Movable, Player, SpriteSize, Velocity};
use player::PlayerPlugin;
mod ball;
pub mod components;
mod player;
// pub mod stats;

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
    commands.spawn_bundle(Camera2dBundle::default());

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
        .add_system(paddle_bounce)
        .run();
}

fn movement_system(mut query: Query<(&Velocity, &mut Transform), With<Movable>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let x = translation.x + velocity.x * TIME_STEP * BASE_SPEED;
        let y = translation.y + velocity.y * TIME_STEP * BASE_SPEED;

        translation.x = x;
        translation.y = y;
    }
}

fn paddle_bounce(
    mut ball_query: Query<
        (&mut Transform, &mut Velocity, &SpriteSize),
        (With<Ball>, Without<Player>),
    >,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    for (mut ball_transform, mut ball_velocity, ball_sprite_size) in ball_query.iter_mut() {
        let ball_scale = Vec2::from(ball_transform.scale.xy());
        for (player_transform, player_sprite_size) in player_query.iter() {
            let player_scale = Vec2::from(ball_transform.scale.xy());

            let collision = collide(
                ball_transform.translation,
                ball_scale * ball_sprite_size.0,
                player_transform.translation,
                player_scale * player_sprite_size.0,
            );

            if let Some(col) = collision {
                match col {
                    Collision::Right => ball_velocity.x *= -1.,
                    Collision::Left => ball_velocity.x *= -1.,
                    Collision::Top => ball_velocity.y *= -1.,
                    Collision::Bottom => ball_velocity.y *= -1.,
                    Collision::Inside => ball_transform.translation.x = 0.,
                }
            }
        }
    }
}
