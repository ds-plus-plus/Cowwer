use bevy::prelude::*;

use crate::song_manager;

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    music: Single<Entity, With<song_manager::Music>>,
) {
    commands.entity(music.entity()).despawn();

    let font = asset_server.load("fonts/Chillax.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 100.0,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
        ..default()
    };

    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("Thanks for playing\n-PixelDS"),
        text_font.clone(),
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}
