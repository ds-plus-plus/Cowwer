use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player, tilemap};

#[derive(Component)]
pub struct Crate;

#[derive(Resource)]
pub struct CrateHandle(pub Handle<Image>);

pub const CRATE_MASK: u32 = 0b0110;

pub fn get_random_range_secure(min: u32, max: u32) -> u32 {
    if max <= min {
        return min;
    }
    let range = max - min;

    let mut bytes = [0u8; 4]; // Use 4 bytes for a u32 range
    getrandom::fill(&mut bytes).expect("Failed to get random bytes");

    let value = u32::from_ne_bytes(bytes);

    value % range + min
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CrateHandle(
        asset_server.load("tilemap/tiles/level1/crate.png"),
    ));
}

pub fn spawn_crate(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &tilemap::Tilemap,
) {
    commands.spawn((
        Crate,
        tilemap::Decayable,
        RigidBody::Fixed,
        Collider::cuboid(tilemap.width as f32 / 2., tilemap.height as f32 / 2.),
        Sensor,
        CollisionGroups::new(
            Group::from_bits_truncate(CRATE_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK),
        ),
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        Sprite::from_image(handle.clone()),
        Transform::from_translation(tilemap.grid_to_world(x, y, 0.0))
            .with_scale(Vec3::splat(tilemap.scale as f32)),
    ));
}

pub fn crate_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<&Transform, With<Sensor>>,
    crates: Query<(), With<Crate>>,
    players: Query<(), With<player::Player>>,
    mut player: Single<
        (
            &mut player::Supercharged,
            &mut player::MudMode,
            &mut player::Invinciblity,
            &mut player::Uncontrollable,
            &mut Sprite,
        ),
        With<player::Player>,
    >,
    mut move_timer: ResMut<player::MoveTimer>,
    mut mud_timer: ResMut<player::MudTimer>,
    mut invincibility_timer: ResMut<player::InvincibilityTimer>,
    mut supercharge_timer: ResMut<player::SuperchargeTimer>,
    mut uncontrollable_timer: ResMut<player::UncontrollableTimer>,
    uncontrollable_handle: Res<player::UncontrollableHandle>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && crates.contains(other_ent) {
                    commands.entity(other_ent).despawn();

                    match get_random_range_secure(0, 20) {
                        i if i <= 5 => {
                            player.0.0 = true;
                            move_timer.0.set_duration(Duration::from_secs_f32(0.15));
                            supercharge_timer.0.reset();
                            supercharge_timer.0.unpause();
                        }
                        i if i >= 5 && i <= 10 => {
                            player.1.0 = true;
                            move_timer.0.set_duration(Duration::from_secs_f32(0.9));
                            mud_timer.0.reset();
                            mud_timer.0.unpause();
                        }
                        i if i > 10 && i <= 15 => {
                            player.2.0 = true;
                            player.4.color = Color::srgba(1.0, 1.0, 1.0, 0.4);
                            invincibility_timer.0.reset();
                            invincibility_timer.0.unpause();
                        }
                        i if i > 15 => {
                            player.3.0 = true;
                            player.4.image = uncontrollable_handle.0.clone();
                            uncontrollable_timer.0.reset();
                            uncontrollable_timer.0.unpause();
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
}