#![allow(clippy::type_complexity)]

use std::time::{Duration, Instant};

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use leafwing_input_manager::prelude::*;

mod input;
use input::Action;

mod tanks;
use tanks::{AimWithMouse, Barrel, ShotCooldown, Tank};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: if cfg!(debug_assertions) {
                    Level::DEBUG
                } else {
                    Level::INFO
                },
                ..default()
            }),
            InputManagerPlugin::<Action>::default(),
        ))
        .insert_resource(ShotCooldown(Duration::from_secs(5)))
        .add_systems(Startup, (setup_player, setup_camera))
        // Chaining is a bit of a hack. For some reason, putting aim_with_cursor in PreUpdate means the action data won't propagate to handle_movement
        .add_systems(
            Update,
            (input::aim_with_cursor, tanks::handle_tank_movement).chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(mut commands: Commands, ass: Res<AssetServer>, shot_cooldown: Res<ShotCooldown>) {
    let barrel_id = commands
        .spawn((
            Barrel,
            SpriteBundle {
                texture: ass.load("kenney_top-down-tanks/PNG/Tanks/barrelBlue_outline.png"),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        Tank {
            last_shot: Instant::now() - shot_cooldown.0,
            barrel_id,
            pivot_offset: Vec2::new(2.0, 0.0),
            speed: 300.0,
        },
        InputManagerBundle::<Action> {
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(VirtualDPad::wasd(), Action::Move)
                .insert(GamepadButtonType::RightTrigger2, Action::Shoot)
                .insert(GamepadButtonType::East, Action::Shoot)
                .insert(QwertyScanCode::Space, Action::Shoot)
                .insert(DualAxis::right_stick(), Action::Aim)
                .build(),
            ..default()
        },
        AimWithMouse,
        SpriteBundle {
            texture: ass.load("kenney_top-down-tanks/PNG/Tanks/tankBlue_outline.png"),
            ..default()
        },
    ));
}
