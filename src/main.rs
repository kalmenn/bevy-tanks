#![allow(clippy::type_complexity)]

use std::time::{Duration, Instant};

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

mod input;
use input::Action;

use leafwing_input_manager::prelude::*;

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
        .add_systems(Update, (input::aim_with_cursor, handle_movement).chain())
        .run();
}

#[derive(Debug, Clone, Component)]
pub struct Tank {
    /// The last [`Instant`] at which a shot was fired.
    /// The player can shoot again if the [`Duration`] set by the [`ShotCooldown`] ressource has elapsed since.
    last_shot: Instant,
    /// The [`Entity`] representing the barrel attached to this tank.
    /// Used for setting the transform of the barrel
    barrel_id: Entity,
    pivot_offset: Vec2,
    speed: f32,
}

impl Tank {
    fn global_pivot_position(&self, tank_transform: &Transform) -> Vec2 {
        (tank_transform.translation.truncate())
            + (tank_transform.local_x().truncate() * self.pivot_offset.x)
            + (tank_transform.local_y().truncate() * self.pivot_offset.y)
    }
}

#[derive(Component)]
pub struct AimWithMouse;

#[derive(Component)]
struct Barrel;

#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct ShotCooldown(Duration);

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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_movement(
    mut q_body: Query<(&mut Transform, &ActionState<Action>, &Tank)>,
    mut q_barrels: Query<&mut Transform, (Without<Tank>, With<Barrel>)>,
    time: Res<Time>,
) {
    for (ref mut body_transform, action_state, tank) in q_body.iter_mut() {
        if action_state.pressed(Action::Move) {
            let axis_pair = action_state
                .axis_pair(Action::Move)
                .expect("an axis pair should be configured for this action");

            let angle = axis_pair.rotation().unwrap_or_default().into_radians();
            body_transform.rotation = Quat::from_rotation_z(angle);

            let movement = body_transform.local_x()
                * axis_pair.length().min(1.0)
                * tank.speed
                * time.delta_seconds();
            body_transform.translation += movement;
        }

        if let Ok(ref mut barrel_transform) = q_barrels.get_mut(tank.barrel_id) {
            // let pivot_position = body_transform.translation + (body_transform.local_x() * 2.0);
            let pivot_position = tank.global_pivot_position(body_transform).extend(0.0);

            if let Some(rotation) = action_state
                .axis_pair(Action::Aim)
                .and_then(|axis_pair| axis_pair.rotation())
            {
                barrel_transform.rotation = Quat::from_rotation_z(rotation.into_radians());
            }

            barrel_transform.translation = barrel_transform.local_x() * 19.0 + pivot_position;
        } else {
            error!("a barrel should exist for this tank"); // TODO: Identify tank in error message
        };
    }
}
