use bevy::prelude::*;

use crate::level_manager;

#[derive(Component)]
pub struct DespawnLater;

pub fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/Chillax.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 40.0,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
        ..default()
    };

    commands.spawn((DespawnLater, Camera2d));

    commands.spawn((
        DespawnLater,
        Text2d::new("PRESS SPACE TO PLAY\n"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0., 0., 0.),
    ));
}

pub fn wait_for_start(
    mut commands: Commands,
    despawn_query: Query<Entity, With<DespawnLater>>,
    mut next_state: ResMut<NextState<level_manager::GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for entity in &despawn_query {
            commands.entity(entity).despawn();
        }

        next_state.set(level_manager::GameState::Level1);
    }
}
