pub mod components;
pub mod spawn;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (car_input, car_physics, wheel_steering, car_tilt).chain(),
        );
    }
}
