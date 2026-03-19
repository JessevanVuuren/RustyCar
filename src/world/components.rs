use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use bevy::{math::usize, prelude::*};
use rand::{RngExt, rngs::SmallRng};

pub const TILE_SIZE: f32 = 4.0;

#[derive(Resource, Clone, Debug)]
pub struct StaticWorld {
    pub blocks: Vec<WorldBlock>,
}

#[derive(Clone, Debug)]
pub struct WorldBlock {
    pub objects: Vec<Model>,
    pub surface: Vec<Surface>,
}

#[derive(Clone, Debug, Default)]
pub struct Surface {
    pub positive: Range<TilePos>,
    pub negative: Range<TilePos>,
}

#[derive(Clone, Debug, Default)]
pub struct Model {
    pub placement: Placement,
    pub tile_type: TileType,
    pub path: String,
    pub range: Range<i32>,
    pub comp: Comp,
}

#[derive(Clone, Debug, Default)]
pub enum Range<T> {
    Range(T, T),
    One(T),
    #[default]
    None,
}
#[derive(Clone, Debug, Default)]
pub enum TileType {
    #[default]
    Object,
    Ground,
}

#[derive(Clone, Debug, Default)]
pub struct Placement {
    pub rotation: Rotation,
    pub amount: Value<i32>,
    pub offset: Offset,
    pub scale: Value<f32>,
}

#[derive(Clone, Debug, Default)]
pub enum Offset {
    Fixed(Vec3),
    Random,
    #[default]
    Zero,
}

#[derive(Clone, Debug, Default)]
pub enum Rotation {
    Random(f32, f32),
    Amount(f32, Dir3),
    RandomDirection,
    #[default]
    True,
}

#[derive(Clone, Debug, Default)]
pub enum Value<T> {
    Random(T, T),
    Amount(T),
    #[default]
    True,
}

#[derive(Component)]
pub struct Grass;

#[derive(Component, Clone, Debug, Default)]
pub enum Comp {
    Mushroom,
    Flower,
    Grass(GrassConfig),
    Fence,
    Tree,
    Rock,
    Dirt,
    Log,
    #[default]
    None,
}

#[derive(Clone, Debug)]
pub struct GrassConfig {
    pub color: Noise<Color>,
    pub height: Noise<f32>,
    pub colors: Vec<Color>,
    pub subdivisions: u8,
}

#[derive(Clone, Debug)]
pub struct Noise<T> {
    pub octaves: Vec<NoiseLevel>,
    pub value_1: T,
    pub value_2: T,
}

#[derive(Clone, Debug)]
pub struct NoiseLevel {
    pub frequency: f32,
    pub amplitude: f32,
}

#[derive(Resource, Default, Debug)]
pub struct TileWorld {
    pub ground: HashMap<TilePos, Ground>,
    pub object: HashMap<TilePos, Vec<Entity>>,
}

#[derive(Clone, Debug)]
pub struct Ground {
    pub entity: Entity,
    pub id: isize,
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

    pub fn row_major(self, other: TilePos) -> impl Iterator<Item = TilePos> {
        (self.z..=other.z).flat_map(move |z| (self.x..=other.x).map(move |x| TilePos { x, z }))
    }

    pub fn column_major(self, other: TilePos) -> impl Iterator<Item = TilePos> {
        (self.x..=other.x).flat_map(move |x| (self.z..=other.z).map(move |z| TilePos { x, z }))
    }

    pub fn random_tile_offset(rng: &mut SmallRng) -> Vec3 {
        let half = TILE_SIZE / 2.0;
        Vec3 {
            x: rng.random_range(-half..half),
            y: 0.0,
            z: rng.random_range(-half..half),
        }
    }

    pub fn subtract_range(
        positive: impl Iterator<Item = TilePos>,
        negative: impl Iterator<Item = TilePos>,
    ) -> impl Iterator<Item = TilePos> {
        let positive: Vec<TilePos> = positive.collect();
        let negative: HashSet<TilePos> = negative.collect();

        positive
            .into_iter()
            .filter(move |tile| !negative.contains(tile))
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

impl Sub for TilePos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}

impl Add<i32> for TilePos {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self {
            x: self.x + other,
            z: self.z + other,
        }
    }
}

impl Add for TilePos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}
