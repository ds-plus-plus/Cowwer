use bevy::prelude::*;

#[derive(Component)]
pub struct Music;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Music,
        AudioPlayer::new(asset_server.load("audio/cowwer_theme.ogg")),
        PlaybackSettings::LOOP,
    ));
}
