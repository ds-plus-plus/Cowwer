use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    obstacles, random_crate,
    tilemap::{self, TilemapSpawned},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Alive(pub bool);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Supercharged(pub bool);

#[derive(Component)]
pub struct MudMode(pub bool);

#[derive(Component)]
pub struct Invinciblity(pub bool);

#[derive(Component)]
pub struct Uncontrollable(pub bool);

#[derive(Resource)]
pub struct PlayerHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct MoveTimer(pub Timer);

#[derive(Resource)]
pub struct MoveSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct RespawnSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct SuperchargeHandles {
    pub frame_1: Handle<Image>,
    pub frame_2: Handle<Image>,
}

#[derive(Resource)]
pub struct MudTimer(pub Timer);

#[derive(Resource)]
pub struct InvincibilityTimer(pub Timer);

#[derive(Resource)]
pub struct MudHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct SuperchargeTimer(pub Timer);

#[derive(Resource)]
pub struct UncontrollableHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct UncontrollableTimer(pub Timer);

#[derive(Resource)]
pub struct UncontrollableMoveTimer(pub Timer);

#[derive(Resource)]
pub struct SuperchargeAnimationTimer(pub Timer);

pub const PLAYER_MASK: u32 = 0b0001;

pub fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(PlayerHandle(
        asset_server.load("tilemap/tiles/player/player.png"),
    ));
    commands.insert_resource(MoveSound(asset_server.load("audio/footstep.ogg")));
    commands.insert_resource(RespawnSound(asset_server.load("audio/respawn.ogg")));
    commands.insert_resource(MudHandle(
        asset_server.load("tilemap/tiles/player/mud_player.png"),
    ));
    commands.insert_resource(UncontrollableHandle(
        asset_server.load("tilemap/tiles/player/uncontrollable_player.png"),
    ));
    commands.insert_resource(UncontrollableMoveTimer(Timer::from_seconds(
        0.35,
        TimerMode::Once,
    )));

    commands.insert_resource(SuperchargeAnimationTimer(Timer::from_seconds(
        0.1,
        TimerMode::Once,
    )));

    let mut supercharge_timer = SuperchargeTimer(Timer::from_seconds(2.5, TimerMode::Once));
    supercharge_timer.0.pause();
    commands.insert_resource(supercharge_timer);

    let mut mud_timer = MudTimer(Timer::from_seconds(2.5, TimerMode::Once));
    mud_timer.0.pause();
    commands.insert_resource(mud_timer);

    let mut invincibility_timer = InvincibilityTimer(Timer::from_seconds(2.5, TimerMode::Once));
    invincibility_timer.0.pause();
    commands.insert_resource(invincibility_timer);

    let mut uncontrollable_timer = UncontrollableTimer(Timer::from_seconds(2.5, TimerMode::Once));
    uncontrollable_timer.0.pause();
    commands.insert_resource(uncontrollable_timer);
}

pub fn spawn_player(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &tilemap::Tilemap,
) {
    commands.spawn((
        Player,
        Alive(true),
        Speed(4000.),
        Supercharged(false),
        MudMode(false),
        Invinciblity(false),
        Uncontrollable(false),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        Collider::ball(2.), // smaller hitbox to make it fair
        Velocity::zero(),
        CollisionGroups::new(
            Group::from_bits_truncate(PLAYER_MASK),
            Group::from_bits_truncate(random_crate::CRATE_MASK)
                | Group::from_bits_truncate(tilemap::VOID_MASK)
                | Group::from_bits_truncate(obstacles::LANDMINE_MASK)
                | Group::from_bits_truncate(obstacles::DANCING_SPIKE_MASK)
                | Group::from_bits_truncate(tilemap::NEXT_LEVEL_MASK)
                | Group::from_bits_truncate(obstacles::ELECTRICITY_MASK)
                | Group::from_bits_truncate(obstacles::PRESSURE_PLATE_MASK)
                | Group::from_bits_truncate(obstacles::WRECKING_BALL_MASK),
        ),
        ActiveCollisionTypes::all(),
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(
            ((x * tilemap.width) as f32 + (tilemap.width / 2) as f32) * tilemap.scale as f32,
            ((y * tilemap.height) as f32 + (tilemap.height / 2) as f32) * tilemap.scale as f32,
            1.,
        )
        .with_scale(Vec3::splat(tilemap.scale as f32 * 0.75))
        .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
    ));
}

pub fn movement(
    mut commands: Commands,
    mut player: Single<
        (
            &mut Velocity,
            &Speed,
            &mut Transform,
            &mut KinematicCharacterController,
            &Uncontrollable,
        ),
        With<Player>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: ResMut<Time>,
    mut move_timer: ResMut<MoveTimer>,
    move_sound: Res<MoveSound>,
) {
    player.0.linvel.y = 0.;
    player.0.linvel.x = 0.;

    move_timer.0.tick(time.delta());

    if (keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp))
        && move_timer.0.finished()
    {
        commands.spawn(AudioPlayer::new(move_sound.0.clone()));
        player.0.linvel.y = player.1.0 * 100. * time.delta_secs();
        player.2.rotation = Quat::IDENTITY;
        move_timer.0.reset();
    }
    if (keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft))
        && move_timer.0.finished()
    {
        commands.spawn(AudioPlayer::new(move_sound.0.clone()));
        player.0.linvel.x = -player.1.0 * 100. * time.delta_secs();
        player.2.rotation = Quat::from_rotation_z(PI / 2.);
        move_timer.0.reset();
    }
    if (keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown))
        && move_timer.0.finished()
    {
        commands.spawn(AudioPlayer::new(move_sound.0.clone()));
        player.0.linvel.y = -player.1.0 * 100. * time.delta_secs();
        player.2.rotation = Quat::from_rotation_z(-PI);
        move_timer.0.reset();
    }
    if (keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight))
        && move_timer.0.finished()
    {
        commands.spawn(AudioPlayer::new(move_sound.0.clone()));
        player.0.linvel.x = player.1.0 * 100. * time.delta_secs();
        player.2.rotation = Quat::from_rotation_z(-PI / 2.);
        move_timer.0.reset();
    }

    player.3.translation = Some(player.0.linvel * time.delta_secs()); // makes collisions work like magic
}

pub fn life_system(
    mut commands: Commands,
    mut player: Single<
        (
            &mut Transform,
            Entity,
            &Alive,
            &mut Supercharged,
            &mut MudMode,
            &mut Invinciblity,
            &mut Uncontrollable,
        ),
        (With<Player>, Without<Sensor>, Without<tilemap::Void>),
    >,
    mut tilemap_spawned: ResMut<TilemapSpawned>,
    tile_query: Query<Entity, With<tilemap::Decayable>>,
    void_query: Query<Entity, (With<tilemap::Void>, Without<tilemap::Decayable>)>,
    mut decaying_tile: ResMut<tilemap::DecayingTile>,
    mut move_timer: ResMut<MoveTimer>,
    mut decay_time: ResMut<tilemap::DecayTime>,
    mut supercharge_timer: ResMut<SuperchargeTimer>,
    mut mud_timer: ResMut<MudTimer>,
    mut invincibility_timer: ResMut<InvincibilityTimer>,
    mut uncontrollable_timer: ResMut<UncontrollableTimer>,
    respawn_sound: Res<RespawnSound>,
) {
    if !player.2.0 {
        commands.spawn(AudioPlayer::new(respawn_sound.0.clone()));
        tilemap_spawned.0 = false;
        decaying_tile.0 = 0.;
        move_timer.0.reset();
        move_timer.0.set_duration(Duration::from_secs_f32(0.35));
        decay_time.0 = 3.;

        player.3.0 = false;
        player.4.0 = false;
        player.5.0 = false;
        player.6.0 = false;
        supercharge_timer.0.reset();
        supercharge_timer.0.pause();
        mud_timer.0.reset();
        mud_timer.0.pause();
        invincibility_timer.0.reset();
        mud_timer.0.pause();
        uncontrollable_timer.0.reset();
        uncontrollable_timer.0.pause();

        for tile in tile_query {
            commands.entity(tile).despawn();
        }

        for void in void_query {
            commands.entity(void).despawn();
        }

        commands.entity(player.1).despawn();
    }
}

pub fn supercharge_animation_system(
    mut player: Single<(&mut Sprite, &mut Supercharged, &Alive), With<Player>>,
    time: Res<Time>,
    player_handle: Res<PlayerHandle>,
    mut move_timer: ResMut<MoveTimer>,
    supercharge_handles: Res<SuperchargeHandles>,
    mut supercharge_timer: ResMut<SuperchargeTimer>,
    mut supercharge_animation_timer: ResMut<SuperchargeAnimationTimer>,
) {
    if supercharge_timer.0.paused() {
        return;
    }

    supercharge_animation_timer.0.tick(time.delta());
    supercharge_timer.0.tick(time.delta());

    if supercharge_timer.0.elapsed_secs() > 0. {
        if supercharge_animation_timer.0.just_finished() {
            match &player.0.image {
                i if i == &supercharge_handles.frame_1 => {
                    player.0.image = supercharge_handles.frame_2.clone()
                }
                i if i == &supercharge_handles.frame_2 => {
                    player.0.image = supercharge_handles.frame_1.clone()
                }
                _ => player.0.image = supercharge_handles.frame_1.clone(),
            }

            supercharge_animation_timer.0.reset();
        }
    }

    if supercharge_timer.0.just_finished() || !player.2.0 {
        supercharge_timer.0.pause();
        player.0.image = player_handle.0.clone();
        player.1.0 = false;
        move_timer.0.set_duration(Duration::from_secs_f32(0.4));
    }
}

pub fn mud_mode(
    mut player: Single<(&mut Sprite, &mut MudMode, &mut Speed, &Alive), With<Player>>,
    time: Res<Time>,
    player_handle: Res<PlayerHandle>,
    mud_handle: Res<MudHandle>,
    mut mud_timer: ResMut<MudTimer>,
    mut move_timer: ResMut<MoveTimer>,
) {
    if mud_timer.0.paused() {
        return;
    }

    mud_timer.0.tick(time.delta());

    if mud_timer.0.elapsed_secs() > 0. {
        player.0.image = mud_handle.0.clone();
    }

    if mud_timer.0.just_finished() || !player.3.0 {
        mud_timer.0.pause();
        player.0.image = player_handle.0.clone();
        move_timer.0.set_duration(Duration::from_secs_f32(0.4));
    }
}

pub fn invincible_handler(
    mut player: Single<(&mut Sprite, &mut Invinciblity, &Alive), With<Player>>,
    time: Res<Time>,
    mut invincible_timer: ResMut<InvincibilityTimer>,
) {
    if invincible_timer.0.paused() {
        return;
    }

    invincible_timer.0.tick(time.delta());

    if invincible_timer.0.just_finished() || !player.2.0 {
        invincible_timer.0.pause();
        player.0.color = Color::srgba(1.0, 1.0, 1.0, 1.);
        player.1.0 = false;
    }
}

pub fn uncontrollable_handler(
    mut commands: Commands,
    mut player: Single<
        (
            &mut Sprite,
            &mut Uncontrollable,
            &mut Velocity,
            &Speed,
            &mut Transform,
            &mut KinematicCharacterController,
            &Alive,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut uncontrollable_timer: ResMut<UncontrollableTimer>,
    player_handle: Res<PlayerHandle>,
    mut uncontrollable_move_timer: ResMut<UncontrollableMoveTimer>,
    move_sound: Res<MoveSound>,
) {
    let dt = time.delta_secs().clamp(0.0, 1.0 / 30.0);

    if uncontrollable_timer.0.paused() {
        return;
    }

    uncontrollable_timer.0.tick(time.delta());
    uncontrollable_move_timer.0.tick(time.delta());

    if uncontrollable_move_timer.0.finished() {
        commands.spawn(AudioPlayer::new(move_sound.0.clone()));

        player.5.translation = Some(Vec2::new(player.3.0 * dt, 0.0));
        player.4.rotation = Quat::from_rotation_z(-PI / 2.);
        uncontrollable_move_timer.0.reset();
    }

    if uncontrollable_timer.0.just_finished() || !player.6.0 {
        uncontrollable_move_timer.0.pause();
        uncontrollable_timer.0.pause();
        player.0.image = player_handle.0.clone();
        player.1.0 = false;
    }
}

pub fn get_player_and_other(
    e1: Entity,
    e2: Entity,
    players: &Query<(), With<Player>>,
) -> Option<(Entity, Entity)> {
    if players.contains(e1) {
        Some((e1, e2))
    } else if players.contains(e2) {
        Some((e2, e1))
    } else {
        None
    }
}
