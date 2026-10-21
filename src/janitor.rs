use bevy::prelude::*;

pub fn sanitise(
    mut commands: Commands,
    sound_query: Query<(Entity, &AudioSink), With<AudioPlayer>>,
) {
    for sound in sound_query {
        if sound.1.empty() {
            commands.entity(sound.0.entity()).despawn();
        }
    }
}
