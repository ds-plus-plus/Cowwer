use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player;
use crate::tilemap::Decayable;
use crate::tilemap::Tilemap;

#[derive(Component)]
pub struct Landmine;

#[derive(Component)]
pub struct DancingSpike;

#[derive(Component)]
pub struct SpikeDirectionSensor;

#[derive(Component)]
pub struct Electricity;

#[derive(Component)]
pub struct PressurePlate;

#[derive(Component)]
pub struct WreckingBall;

#[derive(Resource)]
pub struct WreckingBallHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct PressurePlateHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct LandmineHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct ExplosionSoundHandle(Handle<AudioSource>);

#[derive(Resource)]
pub struct DancingSpikeHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct ElectricityHandles {
    pub frame_1: Handle<Image>,
    pub frame_2: Handle<Image>,
    pub frame_3: Handle<Image>,
}

#[derive(Resource)]
pub struct ShockSoundHandle(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ClickSoundHandle(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ElectricityAnimationTimer(pub Timer);

#[derive(Component)]
pub struct SpikeDirection(pub f32);

#[derive(Resource)]
pub struct MoveSpikeTimer(pub Timer);

pub const LANDMINE_MASK: u32 = 0b1110;
pub const DANCING_SPIKE_MASK: u32 = 0b1111;
pub const SPIKE_DIRECTION_MASK: u32 = 0b11111;
pub const ELECTRICITY_MASK: u32 = 0b1111111;
pub const PRESSURE_PLATE_MASK: u32 = 0b11111111;
pub const WRECKING_BALL_MASK: u32 = 0b111111111;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MoveSpikeTimer(Timer::from_seconds(
        0.4,
        TimerMode::Repeating,
    )));
    commands.insert_resource(LandmineHandle(
        asset_server.load("tilemap/tiles/level1/landmine.png"),
    ));
    commands.insert_resource(DancingSpikeHandle(
        asset_server.load("tilemap/tiles/level1/dancing_spike.png"),
    ));
    commands.insert_resource(PressurePlateHandle(
        asset_server.load("tilemap/tiles/level3/pressure_plate.png"),
    ));
    commands.insert_resource(WreckingBallHandle(
        asset_server.load("tilemap/tiles/level3/wrecking_ball.png"),
    ));
    commands.insert_resource(ExplosionSoundHandle(
        asset_server.load("audio/explosion.ogg"),
    ));
    commands.insert_resource(ClickSoundHandle(asset_server.load("audio/click.ogg")));
    commands.insert_resource(ElectricityHandles {
        frame_1: asset_server.load("tilemap/tiles/electricity/electricityl1.png"),
        frame_2: asset_server.load("tilemap/tiles/electricity/electricityl2.png"),
        frame_3: asset_server.load("tilemap/tiles/electricity/electricityl3.png"),
    });
    commands.insert_resource(ShockSoundHandle(asset_server.load("audio/shock.ogg")));
    commands.insert_resource(ElectricityAnimationTimer(Timer::from_seconds(
        0.25,
        TimerMode::Once,
    )));
}

pub fn spawn_landmine(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
) {
    commands.spawn((
        Landmine,
        Decayable,
        RigidBody::Fixed,
        Collider::ball(2.),
        Sensor,
        CollisionGroups::new(
            Group::from_bits_truncate(LANDMINE_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK),
        ),
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(
            ((x * tilemap.width) as f32 + (tilemap.width / 2) as f32) * tilemap.scale as f32,
            ((y * tilemap.height) as f32 + (tilemap.height / 2) as f32) * tilemap.scale as f32,
            0.,
        )
        .with_scale(Vec3::splat(tilemap.scale as f32)),
    ));
}

pub fn spawn_dancing_spike(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
    direction: f32,
) {
    commands.spawn((
        Decayable,
        DancingSpike,
        SpikeDirection(direction),
        Collider::ball(3.),
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(
            ((x * tilemap.width) as f32 + (tilemap.width / 2) as f32) * tilemap.scale as f32,
            ((y * tilemap.height) as f32 + (tilemap.height / 2) as f32) * tilemap.scale as f32,
            0.,
        )
        .with_scale(Vec3::splat(tilemap.scale as f32)),
        Sensor,
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(
            Group::from_bits_truncate(DANCING_SPIKE_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK)
                | Group::from_bits_truncate(SPIKE_DIRECTION_MASK),
        ),
    ));
}

pub fn spawn_spike_sensor(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
) {
    commands.spawn((
        Decayable,
        SpikeDirectionSensor,
        Collider::cuboid(tilemap.width as f32 / 2., tilemap.height as f32 / 2.),
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(
            ((x * tilemap.width) as f32 + (tilemap.width / 2) as f32) * tilemap.scale as f32,
            ((y * tilemap.height) as f32 + (tilemap.height / 2) as f32) * tilemap.scale as f32,
            -0.,
        )
        .with_scale(Vec3::splat(tilemap.scale as f32)),
        Sensor,
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(
            Group::from_bits_truncate(SPIKE_DIRECTION_MASK),
            Group::from_bits_truncate(DANCING_SPIKE_MASK),
        ),
    ));
}

pub fn spawn_electricity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
) {
    commands.spawn((
        Decayable,
        Electricity,
        Collider::cuboid(tilemap.width as f32 / 2., tilemap.height as f32 / 2.),
        Sprite::from_image(asset_server.load("tilemap/tiles/electricity/electricityl1.png")),
        Transform::from_xyz(
            ((x * tilemap.width) as f32 + (tilemap.width / 2) as f32) * tilemap.scale as f32,
            ((y * tilemap.height) as f32 + (tilemap.height / 2) as f32) * tilemap.scale as f32,
            -0.,
        )
        .with_scale(Vec3::splat(tilemap.scale as f32)),
        Sensor,
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(
            Group::from_bits_truncate(ELECTRICITY_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK),
        ),
    ));
}

pub fn spawn_pressure_plate(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: usize,
    y: usize,
    tilemap: &Tilemap,
) {
    commands.spawn((
        PressurePlate,
        Decayable,
        RigidBody::Fixed,
        Collider::cuboid(tilemap.width as f32 / 2., tilemap.height as f32 / 2.),
        Sensor,
        CollisionGroups::new(
            Group::from_bits_truncate(PRESSURE_PLATE_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK),
        ),
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(
            ((x * tilemap.width) as f32 + (tilemap.width / 2) as f32) * tilemap.scale as f32,
            ((y * tilemap.height) as f32 + (tilemap.height / 2) as f32) * tilemap.scale as f32,
            0.,
        )
        .with_scale(Vec3::splat(tilemap.scale as f32)),
    ));
}

pub fn spawn_wrecking_ball(
    commands: &mut Commands,
    handle: Handle<Image>,
    x: f32,
    y: f32,
    scale: f32,
) {
    commands.spawn((
        WreckingBall,
        Decayable,
        RigidBody::Dynamic,
        Collider::ball(4.),
        Velocity::linear(Vec2::new(-3000., 100.)),
        Restitution::coefficient(1.0),
        CollisionGroups::new(
            Group::from_bits_truncate(WRECKING_BALL_MASK),
            Group::from_bits_truncate(player::PLAYER_MASK),
        ),
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(x, y, 1.).with_scale(Vec3::splat(scale)),
    ));
}

pub fn pressure_plate_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<&Transform, With<Sensor>>,
    pressure_plates: Query<&Transform, With<PressurePlate>>,
    players: Query<(), With<player::Player>>,
    click_handle: Res<ClickSoundHandle>,
    wrecking_ball_handle: Res<WreckingBallHandle>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                    if triggers.contains(other_ent) && pressure_plates.contains(other_ent) {
                        commands.spawn(AudioPlayer::new(click_handle.0.clone()));
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                    if triggers.contains(other_ent) && pressure_plates.contains(other_ent) {
                        if let Ok(plate) = pressure_plates.get(other_ent) {
                            spawn_wrecking_ball(
                                &mut commands,
                                wrecking_ball_handle.0.clone(),
                                plate.translation.x + 1500.,
                                plate.translation.y,
                                12.,
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn wrecking_ball_system(
    mut collision_events: EventReader<CollisionEvent>,
    wrecking_balls: Query<(), With<WreckingBall>>,
    players: Query<(), With<player::Player>>,
    mut player: Single<(&mut player::Alive, &player::Invinciblity), With<player::Player>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if wrecking_balls.contains(other_ent) && !player.1.0 {
                    player.0.0 = false;
                }
            }
        }
    }
}

pub fn landmine_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<&Transform, With<Sensor>>,
    landmines: Query<(), With<Landmine>>,
    players: Query<(), With<player::Player>>,
    explode_handle: Res<ExplosionSoundHandle>,
    mut player: Single<(&mut player::Alive, &player::Invinciblity), With<player::Player>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && landmines.contains(other_ent) && !player.1.0 {
                    commands.spawn(AudioPlayer::new(explode_handle.0.clone()));
                    player.0.0 = false;
                }
            }
        }
    }
}

pub fn dancing_spike_system(
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<Entity, With<Sensor>>,
    mut spikes: Query<&mut SpikeDirection, With<DancingSpike>>,
    direction_sensor: Query<(), (With<SpikeDirectionSensor>, Without<DancingSpike>)>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (spike_ent, other_ent) = if spikes.contains(*e1) && triggers.contains(*e2) {
                (*e1, *e2)
            } else if spikes.contains(*e2) && triggers.contains(*e1) {
                (*e2, *e1)
            } else {
                continue;
            };

            if direction_sensor.contains(other_ent) {
                if let Ok(mut spike) = spikes.get_mut(spike_ent) {
                    spike.0 *= -1.;
                }
            }
        }
    }
}

pub fn move_spikes(
    mut move_spike_timer: ResMut<MoveSpikeTimer>,
    spikes: Query<(&mut Transform, &mut SpikeDirection), With<DancingSpike>>,
    time: Res<Time>,
) {
    move_spike_timer.0.tick(time.delta());

    for mut spike in spikes {
        if move_spike_timer.0.just_finished() {
            spike.0.translation.y += 4500.0 * spike.1.0 * time.delta_secs();
        }
    }
}

pub fn spike_collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<Entity, With<Sensor>>,
    spikes: Query<(), With<DancingSpike>>,
    mut player: Single<(&mut player::Alive, &player::Invinciblity), With<player::Player>>,
    players: Query<(), With<player::Player>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && spikes.contains(other_ent) && !player.1.0 {
                    player.0.0 = false;
                }
            }
        }
    }
}

pub fn electricity_animation_system(
    electricity_query: Query<&mut Sprite, With<Electricity>>,
    electricity_handles: Res<ElectricityHandles>,
    mut electricity_animation_timer: ResMut<ElectricityAnimationTimer>,
    time: Res<Time>,
) {
    if electricity_animation_timer.0.paused() {
        return;
    }

    electricity_animation_timer.0.tick(time.delta());

    for mut electric in electricity_query {
        if electricity_animation_timer.0.just_finished() {
            match &electric.image {
                i if i == &electricity_handles.frame_1 => {
                    electric.image = electricity_handles.frame_2.clone()
                }
                i if i == &electricity_handles.frame_2 => {
                    electric.image = electricity_handles.frame_3.clone()
                }
                _ => electric.image = electricity_handles.frame_1.clone(),
            }

            electricity_animation_timer.0.reset();
        }
    }
}

pub fn electricity_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    triggers: Query<&Transform, With<Sensor>>,
    electricities: Query<(), With<Electricity>>,
    players: Query<(), With<player::Player>>,
    shock_sound_handle: Res<ShockSoundHandle>,
    mut player: Single<(&mut player::Alive, &player::Invinciblity), With<player::Player>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let Some((_, other_ent)) = player::get_player_and_other(*e1, *e2, &players) {
                if triggers.contains(other_ent) && electricities.contains(other_ent) && !player.1.0 {
                    commands.spawn(AudioPlayer::new(shock_sound_handle.0.clone()));
                    player.0.0 = false;
                }
            }
        }
    }
}
