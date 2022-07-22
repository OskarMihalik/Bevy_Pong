use crate::components::{Movable, Player, SpriteSize, Velocity};
use crate::{WinSize, BASE_SPEED, PLAYER_CUSTOM_SIZE, TIME_STEP};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_limit);
    }
}

fn player_spawn_system(mut commands: Commands, win_size: Res<WinSize>) {
    let left = -win_size.w / 2.;
    let middle = 0.;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.21, 0.34),
                custom_size: Some(Vec2::new(PLAYER_CUSTOM_SIZE.0, PLAYER_CUSTOM_SIZE.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(left, middle, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Movable {
            axis_xy: (false, true),
            bounces: false,
        })
        .insert(Velocity { x: 0., y: 0. })
        .insert(SpriteSize::from(PLAYER_CUSTOM_SIZE));
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y = if kb.pressed(KeyCode::Up) {
            1.
        } else if kb.pressed(KeyCode::Down) {
            -1.
        } else {
            0.
        }
    }
}

fn player_limit(
    win_size: Res<WinSize>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let y = translation.y + velocity.y * TIME_STEP * BASE_SPEED;
        if y + (PLAYER_CUSTOM_SIZE.1 / 2.) > (win_size.h / 2.) {
            velocity.y = 0.;
            translation.y = -(PLAYER_CUSTOM_SIZE.1 / 2.) + (win_size.h / 2.)
        } else if y - (PLAYER_CUSTOM_SIZE.1 / 2.) < (-win_size.h / 2.) {
            velocity.y = 0.;
            translation.y = (PLAYER_CUSTOM_SIZE.1 / 2.) - (win_size.h / 2.)
        }
    }
}
