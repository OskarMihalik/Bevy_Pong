use crate::components::{Ball, Movable, SpriteSize, Stats, Velocity};
use crate::{WinSize, BALL_SIZE, BASE_SPEED, TIME_STEP};
use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ball_spawn_system)
            .add_system(bounce_system);
    }
}

fn ball_spawn_system(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.21, 0.34),
                custom_size: Some(Vec2::new(BALL_SIZE.0, BALL_SIZE.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball)
        .insert(Movable {
            axis_xy: (true, true),
            bounces: true,
        })
        .insert(Velocity { x: 0.95, y: 1. })
        .insert(SpriteSize::from(BALL_SIZE));
}

fn bounce_system(
    win_size: Res<WinSize>,
    mut query: Query<(&mut Velocity, &mut Transform, &Movable), With<Ball>>,
    mut query_stats: Query<&mut Stats>,
) {
    for (mut velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        let x = translation.x + velocity.x * TIME_STEP * BASE_SPEED;
        let y = translation.y + velocity.y * TIME_STEP * BASE_SPEED;

        if movable.bounces {
            if y + (BALL_SIZE.1 / 2.) >= (win_size.h / 2.)
                || y - (BALL_SIZE.1 / 2.) <= (-win_size.h / 2.)
            {
                velocity.y *= -1.;
            } else if x - (BALL_SIZE.0 / 2.) <= (-win_size.w / 2.) {
                query_stats.single_mut().player2 += 1;
                velocity.x *= -1.;
            } else if x + (BALL_SIZE.0 / 2.) >= (win_size.w / 2.) {
                query_stats.single_mut().player1 += 1;
                velocity.x *= -1.;
            }
        }
    }
}
