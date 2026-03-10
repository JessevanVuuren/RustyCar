use std::{collections::HashMap, ops::Sub};

use bevy::prelude::*;
use rand::{RngExt, rngs::SmallRng};

const TILE_SIZE: f32 = 4.0;

#[derive(Resource, Clone, Debug)]
pub struct StaticWorld {
    pub blocks: Vec<WorldBlock>,
}

#[derive(Clone, Debug)]
pub struct WorldBlock {
    pub objects: Vec<Model>,
    pub start: TilePos,
    pub stop: TilePos,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub name: String,
    pub comp: Comp,
    pub amount: u8,
    pub range: u8,
}

#[derive(Component, Clone, Debug)]
pub enum Comp {
    Flower,
    Grass,
    Fence,
    Tree,
    Rock,
    Dirt,
    Log,
}

#[derive(Resource, Default)]
pub struct TileWorld {
    pub ground: HashMap<TilePos, Entity>,
    pub object: HashMap<TilePos, Vec<Entity>>,
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct TilePos {
    pub x: i32,
    pub z: i32,
}

impl TilePos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

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

impl Sub<i32> for TilePos {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        Self {
            x: self.x - other,
            z: self.z - other,
        }
    }
}
