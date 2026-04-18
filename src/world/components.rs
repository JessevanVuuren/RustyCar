use std::{
    collections::{HashMap, HashSet},
    f32::consts::FRAC_PI_2,
    ops::{Add, Sub},
};

use bevy::{ecs::query::QueryData, math::usize, prelude::*, reflect::TypeData};
use rand::{RngExt, rngs::SmallRng};

use crate::{
    animal::components::{AnimalKind, AnimalModel},
    collision::components::ModelCollider,
    extra::components::{Noise, Range, Value},
    world::tile_pos::TilePos,
};

pub const TILE_SIZE: f32 = 4.0;
pub const QUAD_POINTS: i32 = 6;
pub const COLOR_PRECISION: i32 = 1000;
pub const BASE_ASSET: &str = "models/";

pub const DOWN: f32 = 0.0;
pub const UP: f32 = FRAC_PI_2;
pub const LEFT: f32 = FRAC_PI_2 * 3.0;
pub const RIGHT: f32 = FRAC_PI_2 * 2.0;

#[derive(Resource, Clone, Debug)]
pub struct StaticWorld {
    pub blocks: Vec<WorldBlock>,
    pub animals: Vec<AnimalRoam>,
}

#[derive(Debug, Clone)]
pub struct AnimalRoam {
    pub animal: AnimalModel,
    pub surface: Surface,
}

#[derive(Clone, Debug)]
pub struct WorldBlock {
    pub tiletype: TileType,
    pub surface: Surface,
}

#[derive(Clone, Debug, Default)]
pub struct Surface {
    pub positive: Vec<Range<TilePos>>,
    pub negative: Vec<Range<TilePos>>,
}

#[derive(Clone, Debug, Default)]
pub struct WorldModel {
    pub collider: Option<ModelCollider>,
    pub placement: Placement,
    pub path: String,
    pub range: Range<i32>,
    pub comp: Comp,
}

#[derive(Clone, Debug)]
pub enum TileType {
    Patches(Noise<Vec<WorldModel>, (f32, i32)>),
    Models(Vec<WorldModel>),
    Ground(GroundConfig),
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
    RandomInTile,
    RandomRange(Vec3),
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

#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Rock;
#[derive(Component)]
pub struct Mushroom;
#[derive(Component)]
pub struct Flower;
#[derive(Component)]
pub struct Fence;
#[derive(Component)]
pub struct Tree;
#[derive(Component)]
pub struct Log;
#[derive(Component)]
pub struct Object;

#[derive(Component, Clone, Debug, Default)]
pub enum Comp {
    Mushroom,
    Flower,
    Fence,
    Tree,
    Rock,
    Log,
    #[default]
    None,
}

#[derive(Clone, Debug)]
pub struct GroundConfig {
    pub color: Noise<Color, Color>,
    pub height: Noise<f32, f32>,
    pub colors: Vec<Color>,
    pub subdivisions: u8,
    pub color_samples: i32,
    pub color_spread: f32,
    pub stitch_intensity: f32,
    pub stitch_spread: f32,
}

#[derive(Clone, Debug)]
pub struct GroundId {
    pub entity: Entity,
    pub id: usize,
}

#[derive(Resource, Default, Debug)]
pub struct TileWorld {
    pub ground: HashMap<TilePos, GroundId>,
    pub models: HashMap<TilePos, Vec<Entity>>,
}

impl TileWorld {
    pub fn all_entitys_from_range(&self, range: &[TilePos]) -> impl Iterator<Item = Entity> {
        range
            .iter()
            .flat_map(|tile| self.models.get(tile).into_iter().flatten().copied())
    }
}
