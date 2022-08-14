use crate::components::{Movable, Player, PlayerNumber, SpriteSize, Velocity};
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
    let left_1 = -win_size.w / 2.;
    let left_2 = win_size.w / 2.;
    let middle = 0.;
    player_spawner(
        &mut commands,
        left_1,
        middle,
        Player {
            player: PlayerNumber::Player1,
            player_up_down_keys: (KeyCode::W, KeyCode::S),
        },
    );
    player_spawner(
        &mut commands,
        left_2,
        middle,
        Player {
            player: PlayerNumber::Player2,
            player_up_down_keys: (KeyCode::Up, KeyCode::Down),
        },
    );
}

fn player_spawner(commands: &mut Commands, x: f32, y: f32, player: Player) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(PLAYER_CUSTOM_SIZE.0, PLAYER_CUSTOM_SIZE.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player)
        .insert(Movable {
            axis_xy: (false, true),
            bounces: false,
        })
        .insert(Velocity { x: 0., y: 0. })
        .insert(SpriteSize::from(PLAYER_CUSTOM_SIZE));
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in query.iter_mut() {
        velocity.y = if kb.pressed(player.player_up_down_keys.0) {
            1.
        } else if kb.pressed(player.player_up_down_keys.1) {
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
