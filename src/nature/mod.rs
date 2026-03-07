pub mod components;
pub mod spawn;

use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use crate::nature::{
    components::{Model, Nature, World},
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
    let nature_models: Vec<Model> = vec![
        Model {
            name: "rock_0".into(),
            nature: Nature::Rock,
        },
        Model {
            name: "rock_1".into(),
            nature: Nature::Rock,
        },
        Model {
            name: "rock_2".into(),
            nature: Nature::Rock,
        },
        Model {
            name: "rock_3".into(),
            nature: Nature::Rock,
        },
        Model {
            name: "rock_4".into(),
            nature: Nature::Rock,
        },
        Model {
            name: "rock_5".into(),
            nature: Nature::Rock,
        },
        Model {
            name: "pine_tree_1".into(),
            nature: Nature::Tree,
        },
        Model {
            name: "pine_tree_2".into(),
            nature: Nature::Tree,
        },
        Model {
            name: "pine_tree_3".into(),
            nature: Nature::Tree,
        },
        Model {
            name: "pine_tree_4".into(),
            nature: Nature::Tree,
        },
        Model {
            name: "log".into(),
            nature: Nature::Log,
        },
        Model {
            name: "log_1".into(),
            nature: Nature::Log,
        },
        Model {
            name: "log_2".into(),
            nature: Nature::Log,
        },
        Model {
            name: "log_3".into(),
            nature: Nature::Log,
        },
    ];

    let mut world = randomize_objects(&nature_models, 10, 10, 250);
    place_ground_plane(&mut world, 10, 10);
    spawn_environment(&mut commands, &asset_server, world);
}

fn randomize_objects(nature: &Vec<Model>, x_size: u8, z_size: u8, objects: u32) -> Vec<World> {
    let x_offset = x_size as f32 * GROUND_SIZE / 2.0;
    let z_offset = z_size as f32 * GROUND_SIZE / 2.0;
    let mut rng = SmallRng::seed_from_u64(1604);

    let mut world: Vec<World> = Vec::new();

    for _ in 0..objects {
        let nature_object = nature[rng.random_range(0..nature.len())].clone();

        let x = rng.random_range(-x_offset..x_offset);
        let z = rng.random_range(-z_offset..z_offset);
        let y = rng.random_range(-PI..PI);

        world.push(World {
            model: nature_object,
            transform: Transform::from_xyz(x, 0.0, z).with_rotation(Quat::from_rotation_y(y)),
        });
    }

    world
}

fn place_ground_plane(world: &mut Vec<World>, x_size: u8, z_size: u8) {
    let x_offset = x_size as f32 * GROUND_SIZE / 2.0 - (GROUND_SIZE / 2.0);
    let z_offset = z_size as f32 * GROUND_SIZE / 2.0 - (GROUND_SIZE / 2.0);

    for x_index in 0..x_size {
        for z_index in 0..z_size {
            let x = x_index as f32 * GROUND_SIZE - x_offset;
            let z = z_index as f32 * GROUND_SIZE - z_offset;

            world.push(World {
                model: Model {
                    name: "ground".into(),
                    nature: Nature::Ground,
                },
                transform: Transform::from_xyz(x, 0.0, z),
            });
        }
    }
}
