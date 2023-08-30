use std::time::{Instant, Duration};

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::Action;

#[derive(Debug, Clone, Component)]
pub struct Tank {
    /// The last [`Instant`] at which a shot was fired.
    /// The player can shoot again if the [`Duration`] set by the [`ShotCooldown`] ressource has elapsed since.
    pub last_shot: Instant,
    /// The [`Entity`] representing the barrel attached to this tank.
    /// Used for setting the transform of the barrel
    pub barrel_id: Entity,
    pub pivot_offset: Vec2,
    pub speed: f32,
}

impl Tank {
    pub fn global_pivot_position(&self, tank_transform: &Transform) -> Vec2 {
        (tank_transform.translation.truncate())
            + (tank_transform.local_x().truncate() * self.pivot_offset.x)
            + (tank_transform.local_y().truncate() * self.pivot_offset.y)
    }
}

#[derive(Component)]
pub struct AimWithMouse;

#[derive(Component)]
pub struct Barrel;

#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct ShotCooldown(pub Duration);

pub fn handle_tank_movement(
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
