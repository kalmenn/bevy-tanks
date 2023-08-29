use std::time::{Duration, Instant};

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use leafwing_input_manager::prelude::*;

const PLAYER_SPEED: f32 = 500.0;

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
        .add_systems(Update, handle_movement)
        .run();
}

#[derive(Actionlike, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
enum Action {
    Move,
    Shoot,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Component)]
struct Player {
    last_shot: Instant,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            last_shot: Instant::now() - Duration::from_secs(420),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Resource)]
struct ShotCooldown(Duration);

fn setup_player(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn((
        Player::default(),
        InputManagerBundle::<Action> {
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(VirtualDPad::wasd(), Action::Move)
                .insert(GamepadButtonType::RightTrigger2, Action::Shoot)
                .insert(GamepadButtonType::East, Action::Shoot)
                .insert(QwertyScanCode::Space, Action::Shoot)
                .build(),
            ..default()
        },
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
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, action_state) in query.iter_mut() {
        if action_state.pressed(Action::Move) {
            let axis_pair = action_state
                .axis_pair(Action::Move)
                .expect("an axis pair should be configured for this action");

            let angle = axis_pair.rotation().unwrap_or_default().into_radians();
            transform.rotation = Quat::from_rotation_z(angle);

            let movement = transform.local_x()
                * axis_pair.length().min(1.0)
                * PLAYER_SPEED
                * time.delta_seconds();
            transform.translation += movement;
        }
    }
}
