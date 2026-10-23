use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{obstacles, player};

#[derive(Resource)]
pub struct TilemapSpawned(pub bool);

#[derive(Resource)]
pub struct DecayingTile(pub f32);

#[derive(Resource)]
pub struct BackgroundHandle(pub Handle<Image>);

#[derive(Component)]
pub struct Void;

#[derive(Resource)]
pub struct GroundHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct TilemapHandle(pub Handle<Tilemap>);

#[derive(Resource)]
pub struct TilemapTwoHandle(pub Handle<Tilemap>);

#[derive(Resource)]
pub struct TilemapThreeHandle(pub Handle<Tilemap>);

#[derive(Resource)]
pub struct DecayTimer(pub Timer);

#[derive(Resource)]
pub struct DecayTime(pub f32);

#[derive(Component)]
pub struct Decayable;

#[derive(Component)]
pub struct NextLevel;

#[derive(Resource)]
pub struct NextLevelHandle(pub Handle<Image>);

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Tilemap {
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub map: Vec<Vec<u8>>,
}

impl Tilemap {
    pub fn grid_to_world(&self, x: usize, y: usize, z: f32) -> Vec3 {
        Vec3::new(
            ((x * self.width) as f32 + (self.width / 2) as f32) * self.scale as f32,
            ((y * self.height) as f32 + (self.height / 2) as f32) * self.scale as f32,
            z,
        )
    }
}

pub const VOID_MASK: u32 = 0b1000;
pub const NEXT_LEVEL_MASK: u32 = 0b111111;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TilemapHandle(asset_server.load("tilemap/tilemap.json")));
    commands.insert_resource(TilemapTwoHandle(asset_server.load("tilemap/tilemap2.json")));
    commands.insert_resource(TilemapThreeHandle(
        asset_server.load("tilemap/tilemap3.json"),
    ));
    commands.insert_resource(BackgroundHandle(asset_server.load("tilemap/tiles/bg.png")));
    commands.insert_resource(GroundHandle(asset_server.load("tilemap/tiles/ground.png")));
    commands.insert_resource(player::SuperchargeHandles {
        frame_1: asset_server.load("tilemap/tiles/player/player_supercharged/player1.png"),
        frame_2: asset_server.load("tilemap/tiles/player/player_supercharged/player2.png"),
    });
    commands.insert_resource(TilemapSpawned(false));
    commands.insert_resource(DecayingTile(0.));
    commands.insert_resource(DecayTime(2.5));
    commands.insert_resource(NextLevelHandle(
        asset_server.load("tilemap/tiles/next_level.png"),
    ));
}

pub fn spawn_bg_tile(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
) {
    commands.spawn((
        Decayable,
        Sprite::from_image(handle),
        Transform::from_translation(tilemap.grid_to_world(x, y, -2.0))
            .with_scale(Vec3::splat(tilemap.scale as f32)),
    ));
}

pub fn spawn_next_level(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
) {
    commands.spawn((
        Decayable,
        NextLevel,
        Collider::cuboid(tilemap.width as f32 / 2., tilemap.height as f32 / 2.),
        Sprite::from_image(handle.clone()),
        Transform::from_translation(tilemap.grid_to_world(x, y, -1.0))
            .with_scale(Vec3::splat(tilemap.scale as f32)),
        Sensor,
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(
            Group::from_bits_truncate(NEXT_LEVEL_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK),
        ),
    ));
}

pub fn decay_system(
    mut commands: Commands,
    time: Res<Time>,
    mut decay_timer: ResMut<DecayTimer>,
    tile_query: Query<(Entity, &mut Transform), With<Decayable>>,
    tilemap: ResMut<Assets<Tilemap>>,
    tilemap_handle: Res<TilemapHandle>,
    mut current_tile: ResMut<DecayingTile>,
    mut decay_time: ResMut<DecayTime>,
) {
    decay_timer.0.tick(time.delta());

    if decay_timer.0.finished() {
        if let Some(tile_map) = tilemap.get(tilemap_handle.0.id()) {
            for tile in tile_query {
                if tile.1.translation.x == (96. * current_tile.0) + 48. {
                    commands.spawn((
                        Void,
                        Collider::cuboid(tile_map.width as f32 / 2., tile_map.height as f32 / 2.),
                        tile.1
                            .clone()
                            .with_scale(Vec3::splat(tile_map.scale as f32)),
                        Sensor,
                        ActiveCollisionTypes::all(),
                        ActiveEvents::COLLISION_EVENTS,
                        CollisionGroups::new(
                            Group::from_bits_truncate(VOID_MASK),
                            Group::from_bits_truncate(player::PLAYER_MASK)
                                | Group::from_bits_truncate(obstacles::WRECKING_BALL_MASK),
                        ),
                    ));

                    commands.entity(tile.0).despawn();
                }
            }
        }

        current_tile.0 += 1.;
        decay_time.0 *= 0.95;
        decay_timer
            .0
            .set_duration(Duration::from_secs_f32(decay_time.0));
        decay_timer.0.reset();
    }
}

pub fn void_system(
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<&Transform, With<Sensor>>,
    players: Query<(), With<player::Player>>,
    voids: Query<(), With<Void>>,
    mut player: Single<
        (&mut Transform, Entity, &mut player::Alive),
        (With<player::Player>, Without<Sensor>, Without<Void>),
    >,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && voids.contains(other_ent) {
                    player.2.0 = false;
                }
            }
        }
    }
}
