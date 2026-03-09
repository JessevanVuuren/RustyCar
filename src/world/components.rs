use std::collections::HashMap;

use bevy::prelude::*;
use rand::{RngExt, rngs::SmallRng};

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TilePos {
    pub x: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Rock;

#[derive(Component)]
pub struct Log;

#[derive(Component)]
pub struct Flower;

#[derive(Component, Clone, Copy)]
pub struct Ground;

#[derive(Component)]
pub struct Dirt;

#[derive(Component)]
pub struct Fence;

#[derive(Resource, Default)]
pub struct TileMap {
    pub ground: HashMap<TilePos, Entity>,
    pub object: HashMap<TilePos, Vec<Entity>>,
}

const TILE_SIZE: f32 = 3.99634;
// const TILE_SIZE: f32 = 3.99634 * 1.02;

impl TilePos {
    pub fn to_world_transform(self) -> Transform {
        return Transform::from_xyz(
            (self.x as f32 * TILE_SIZE) as f32,
            0.0,
            (self.z as f32 * TILE_SIZE) as f32,
        );
    }

    pub fn to_random_world_transform(self, rng: &mut SmallRng) -> Transform {
        let mut pos = self.to_world_transform();

        let offset = TILE_SIZE / 2.0;
        let offset_x = rng.random_range(-offset..offset);
        let offset_z = rng.random_range(-offset..offset);

        pos.translation += Vec3::new(offset_x, 0.0, offset_z);
        pos
    }

    pub fn transform_to_tile(transform: &Transform) -> TilePos {
        TilePos {
            x: (transform.translation.x / TILE_SIZE) as i32,
            z: (transform.translation.z / TILE_SIZE) as i32,
        }
    }
}
