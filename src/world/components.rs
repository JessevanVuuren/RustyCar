use std::collections::HashMap;

use bevy::prelude::*;

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
pub struct Ground;

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
}
