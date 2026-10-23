use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{level_manager, obstacles, player, random_crate, tilemap};

pub fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawned: ResMut<tilemap::TilemapSpawned>,
    tilemap: ResMut<Assets<tilemap::Tilemap>>,
    tilemap_handle: Res<tilemap::TilemapThreeHandle>,
    ground_handle: Res<tilemap::GroundHandle>,
    player_handle: Res<player::PlayerHandle>,
    background_handle: Res<tilemap::BackgroundHandle>,
    dancing_spike_handle: Res<obstacles::DancingSpikeHandle>,
    crate_handle: Res<random_crate::CrateHandle>,
    pressure_plate_handle: Res<obstacles::PressurePlateHandle>,
    landmine_handle: Res<obstacles::LandmineHandle>,
    next_level_handle: Res<tilemap::NextLevelHandle>,
) {
    if spawned.0 {
        return;
    }

    if asset_server.is_loaded(&tilemap_handle.0) {
        spawned.0 = true;
    }

    if let Some(tile_map) = tilemap.get(tilemap_handle.0.id()) {
        for (y, row) in tile_map.map.iter().rev().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                tilemap::spawn_bg_tile(&mut commands, background_handle.0.clone(), x, y, tile_map);

                match tile {
                    9 => player::spawn_player(&mut commands, player_handle.0.clone(), x, y, tile_map),
                    0 => {}
                    1 => {
                        commands.spawn((
                            tilemap::Decayable,
                            RigidBody::Fixed,
                            Collider::cuboid(tile_map.width as f32 / 2., tile_map.height as f32 / 2.),
                            Restitution::coefficient(0.7),
                            Sprite::from_image(ground_handle.0.clone()),
                            Transform::from_translation(tile_map.grid_to_world(x, y, -1.0))
                                .with_scale(Vec3::splat(tile_map.scale as f32)),
                        ));
                    }
                    2 => obstacles::spawn_electricity(&mut commands, &asset_server, x, y, tile_map),
                    3 => obstacles::spawn_dancing_spike(&mut commands, dancing_spike_handle.0.clone(), x, y, tile_map, -1.),
                    4 => {
                        commands.spawn((
                            tilemap::Decayable,
                            RigidBody::Fixed,
                            Collider::cuboid(tile_map.width as f32 / 2., tile_map.height as f32 / 2.),
                            Sprite::from_image(ground_handle.0.clone()),
                            Transform::from_translation(tile_map.grid_to_world(x, y, -1.0))
                                .with_scale(Vec3::splat(tile_map.scale as f32)),
                        ));
                        obstacles::spawn_spike_sensor(&mut commands, ground_handle.0.clone(), x, y, tile_map);
                    }
                    5 => random_crate::spawn_crate(&mut commands, crate_handle.0.clone(), x, y, tile_map),
                    6 => obstacles::spawn_pressure_plate(&mut commands, pressure_plate_handle.0.clone(), x, y, tile_map),
                    7 => tilemap::spawn_next_level(&mut commands, next_level_handle.0.clone(), x, y, tile_map),
                    8 => obstacles::spawn_landmine(&mut commands, landmine_handle.0.clone(), x, y, tile_map),
                    _ => {}
                }
            }
        }
    }
}

pub fn next_level(
    mut commands: Commands,
    mut next_state: ResMut<NextState<level_manager::GameState>>,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<Entity, With<Sensor>>,
    next_level_tiles: Query<(), With<tilemap::NextLevel>>,
    players: Query<(), With<player::Player>>,
    mut player: Single<&mut player::Alive, With<player::Player>>,
    camera: Single<Entity, With<Camera2d>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && next_level_tiles.contains(other_ent) {
                    player.0 = false;
                    commands.entity(camera.entity()).despawn(); 
                    next_state.set(level_manager::GameState::EndScreen)
                }
            }
        }
    }
}