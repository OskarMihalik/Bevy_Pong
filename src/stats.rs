use crate::components::Stats;
use bevy::prelude::*;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, stats_spawn_system)
            .add_system(stats_update_system);
    }
}

fn stats_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "0    0",
                TextStyle {
                    font: asset_server.load("VCR_OSD_MONO_1.001.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::FlexEnd,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Stats {
            player1: 0,
            player2: 0,
        });
}

fn stats_update_system(mut query: Query<(&mut Text, &Stats)>) {
    for (mut text, stats) in query.iter_mut() {
        text.sections[0].value = format!("{}    {}", stats.player1, stats.player2)
    }
}
