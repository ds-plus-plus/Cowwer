use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{level_manager, obstacles, player, random_crate, tilemap};

pub fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawned: ResMut<tilemap::TilemapSpawned>,
    tilemap: ResMut<Assets<tilemap::Tilemap>>,
    tilemap_handle: Res<tilemap::TilemapTwoHandle>,
    ground_handle: Res<tilemap::GroundHandle>,
    player_handle: Res<player::PlayerHandle>,
    background_handle: Res<tilemap::BackgroundHandle>,
    next_level_handle: Res<tilemap::NextLevelHandle>,
    landmine_handle: Res<obstacles::LandmineHandle>,
    dancing_spike_handle: Res<obstacles::DancingSpikeHandle>,
    crate_handle: Res<random_crate::CrateHandle>,
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
                if tile == 9 {
                    tilemap::spawn_bg_tile(
                        &mut commands,
                        background_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );

                    player::spawn_player(&mut commands, player_handle.0.clone(), x, y, tile_map);
                } else if tile == 0 {
                    tilemap::spawn_bg_tile(
                        &mut commands,
                        background_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );
                } else if tile == 1 {
                    commands.spawn((
                        tilemap::Decayable,
                        RigidBody::Fixed,
                        Collider::cuboid(tile_map.width as f32 / 2., tile_map.height as f32 / 2.),
                        Sprite::from_image(ground_handle.0.clone()),
                        Transform::from_xyz(
                            ((x * tile_map.width) as f32 + (tile_map.width / 2) as f32)
                                * tile_map.scale as f32,
                            ((y * tile_map.height) as f32 + (tile_map.height / 2) as f32)
                                * tile_map.scale as f32,
                            -1.,
                        )
                        .with_scale(Vec3::splat(tile_map.scale as f32)),
                    ));
                } else if tile == 3 {
                    tilemap::spawn_bg_tile(
                        &mut commands,
                        background_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );

                    obstacles::spawn_landmine(
                        &mut commands,
                        landmine_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );
                } else if tile == 4 {
                    tilemap::spawn_bg_tile(
                        &mut commands,
                        background_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );

                    obstacles::spawn_dancing_spike(
                        &mut commands,
                        dancing_spike_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                        -1.,
                    );
                } else if tile == 5 {
                    commands.spawn((
                        tilemap::Decayable,
                        RigidBody::Fixed,
                        Collider::cuboid(tile_map.width as f32 / 2., tile_map.height as f32 / 2.),
                        Sprite::from_image(ground_handle.0.clone()),
                        Transform::from_xyz(
                            ((x * tile_map.width) as f32 + (tile_map.width / 2) as f32)
                                * tile_map.scale as f32,
                            ((y * tile_map.height) as f32 + (tile_map.height / 2) as f32)
                                * tile_map.scale as f32,
                            -1.,
                        )
                        .with_scale(Vec3::splat(tile_map.scale as f32)),
                    ));

                    obstacles::spawn_spike_sensor(
                        &mut commands,
                        ground_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );
                } else if tile == 6 {
                    tilemap::spawn_bg_tile(
                        &mut commands,
                        background_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );

                    random_crate::spawn_crate(
                        &mut commands,
                        crate_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );
                } else if tile == 7 {
                    tilemap::spawn_next_level(
                        &mut commands,
                        next_level_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );
                } else if tile == 8 {
                    tilemap::spawn_bg_tile(
                        &mut commands,
                        background_handle.0.clone(),
                        x,
                        y,
                        tile_map,
                    );

                    obstacles::spawn_electricity(&mut commands, &asset_server, x, y, tile_map);
                }
            }
        }
    }
}

pub fn next_level(
    mut next_state: ResMut<NextState<level_manager::GameState>>,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<Entity, With<Sensor>>,
    next_level_tiles: Query<(), With<tilemap::NextLevel>>,
    players: Query<(), With<player::Player>>,
    mut player: Single<&mut player::Alive, With<player::Player>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && next_level_tiles.contains(other_ent) {
                    player.0 = false;
                    next_state.set(level_manager::GameState::Level3)
                }
            }
        }
    }
}
