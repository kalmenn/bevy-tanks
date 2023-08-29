use std::time::{Duration, Instant};

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .insert_resource(ShotCooldown(Duration::from_secs(5)))
        .add_systems(Startup, (setup_player, setup_camera))
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

fn setup_player(mut commands: Commands) {
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
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
