pub mod components;
pub mod spawn;
pub mod systems;

use bevy::prelude::*;

use crate::environment::{
    components::{Env, Nature},
    spawn::spawn_environment,
};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

const GROUND_SIZE: f32 = 19.9;

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut world: Vec<Env> = vec![
        Env {
            name: "rock_0".into(),
            nature: Nature::Rock,
            ..Default::default()
        },
        Env {
            name: "rock_1".into(),
            nature: Nature::Rock,
            ..Default::default()
        },
    ];

    place_ground_plane(&mut world, 100, 100);

    spawn_environment(&mut commands, &asset_server, world);
}

fn place_ground_plane(world: &mut Vec<Env>, x_size: u8, z_size: u8) {
    let offset = x_size as f32 * GROUND_SIZE / 2.0 - (GROUND_SIZE / 2.0);

    for x_index in 0..x_size {
        for z_index in 0..z_size {
            let x = x_index as f32 * GROUND_SIZE - offset;
            let z = z_index as f32 * GROUND_SIZE - offset;

            world.push(Env {
                name: "ground".into(),
                nature: Nature::Ground,
                transform: Transform::from_xyz(x, 0.0, z),
            });
        }
    }
}
