use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Sample4,
        Transform::from_xyz(1590., 600., 999.).with_scale(Vec3::splat(2.6)),
    ));
}
