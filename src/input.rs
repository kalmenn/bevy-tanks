use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PrimaryWindow, WindowRef},
};
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};

use crate::tanks::{AimWithMouse, Tank};

#[derive(Actionlike, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum Action {
    Move,
    Shoot,
    Aim,
}

pub fn aim_with_cursor(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform)>,
    mut q_tank: Query<(&Tank, &Transform, &mut ActionState<Action>), With<AimWithMouse>>,
) {
    let window = q_window.single();

    let Some(cursor_world_position) = q_cameras
        .into_iter()
        .find(|(camera, _)| {
            matches!(camera.target, RenderTarget::Window(WindowRef::Primary))
        })
        .and_then(|(camera, camera_transform)| window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        ) else {
            return
        };

    for (tank, tank_transform, ref mut action_state) in q_tank.iter_mut() {
        let direction = (cursor_world_position - tank.global_pivot_position(tank_transform))
            .try_normalize()
            .unwrap_or(Vec2::X);

        action_state.action_data_mut(Action::Aim).axis_pair =
            Some(DualAxisData::from_xy(direction));

        action_state.press(Action::Aim);
    }
}
