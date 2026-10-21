mod camera;
mod end_screen;
mod janitor;
mod level1;
mod level2;
mod level3;
mod level_manager;
mod main_menu;
mod obstacles;
mod player;
mod random_crate;
mod song_manager;
mod tilemap;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_rapier2d::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn run_game() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cowwer".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
            JsonAssetPlugin::<tilemap::Tilemap>::new(&[
                "tilemap/tilemap.json",
                "tilemap/tilemap2.json",
            ]),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        })
        .insert_resource(ClearColor(Color::srgb(0.07, 0.03, 0.1)))
        .insert_resource(player::MoveTimer(Timer::from_seconds(
            0.35,
            TimerMode::Once,
        )))
        .insert_resource(tilemap::DecayTimer(Timer::from_seconds(
            1.5,
            TimerMode::Once,
        )))
        // main menu
        .insert_state(level_manager::GameState::MainMenu)
        .add_systems(
            OnEnter(level_manager::GameState::MainMenu),
            main_menu::setup,
        )
        .add_systems(
            Update,
            main_menu::wait_for_start.run_if(in_state(level_manager::GameState::MainMenu)),
        )
        // level 1
        .add_systems(
            OnEnter(level_manager::GameState::Level1),
            (
                tilemap::setup,
                obstacles::setup,
                camera::setup,
                player::setup,
                random_crate::setup,
                level1::setup,
                song_manager::setup,
            )
                .run_if(in_state(level_manager::GameState::Level1))
                .chain(),
        )
        .add_systems(
            Update,
            (
                tilemap::void_system,
                obstacles::landmine_system,
                obstacles::electricity_animation_system,
                obstacles::electricity_system,
                random_crate::crate_system,
            )
                .run_if(not(in_state(level_manager::GameState::MainMenu)
                    .and(not(in_state(level_manager::GameState::EndScreen)))))
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                player::movement,
                player::life_system,
                player::uncontrollable_handler,
                player::supercharge_animation_system,
                player::mud_mode,
                player::invincible_handler,
                obstacles::move_spikes,
                tilemap::decay_system,
                obstacles::dancing_spike_system,
                obstacles::spike_collision_system,
                janitor::sanitise,
            )
                .chain()
                .run_if(
                    not(in_state(level_manager::GameState::MainMenu))
                        .and(not(in_state(level_manager::GameState::EndScreen))),
                ),
        )
        .add_systems(
            Update,
            (level1::spawn_tilemap, level1::next_level)
                .chain()
                .run_if(in_state(level_manager::GameState::Level1)),
        )
        // level 2
        .add_systems(
            Update,
            ((level2::spawn_tilemap, level2::next_level))
                .chain()
                .run_if(in_state(level_manager::GameState::Level2)),
        )
        // level 3
        .add_systems(
            Update,
            ((level3::spawn_tilemap, level3::next_level))
                .chain()
                .run_if(in_state(level_manager::GameState::Level3)),
        )
        .add_systems(
            FixedUpdate,
            (
                obstacles::pressure_plate_system,
                obstacles::wrecking_ball_system,
            )
                .chain()
                .run_if(in_state(level_manager::GameState::Level3)),
        )
        // end screen
        .add_systems(
            OnEnter(level_manager::GameState::EndScreen),
            end_screen::setup,
        )
        .run();
}
