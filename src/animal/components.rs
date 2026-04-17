use bevy::prelude::*;
use std::collections::HashMap;

use crate::{extra::components::Range, world::tile_pos::TilePos};

#[derive(Clone, Debug)]
pub struct AnimalModel {
    pub amount: i32,
    pub path: String,
    pub kind: AnimalKind,
    pub offset: Transform,
    pub variation: f32,
    pub range: Range<i32>,
    pub animations: Vec<AnimalState>,
    pub behavior: ButterflyBehavior,
}

#[derive(Clone, Debug, Component)]
pub enum ButterflyBehavior {
    FreeFly,
    Swirling,
    FlowerBed,
}

#[derive(Component)]
pub struct FreeFly(pub Vec<TilePos>);

#[derive(Component)]
pub struct Swirling;

#[derive(Component)]
pub struct FlowerBed(pub u8);

#[derive(Component)]
pub struct Butterfly;

#[derive(Resource, Default)]
pub struct AnimalLibrary {
    pub animals: HashMap<String, AnimalAnimations>,
}

#[derive(Component)]
pub struct RestTimer(pub Timer);

#[derive(Component)]
pub struct TargetEntity(pub Entity);

#[derive(Component)]
pub struct TargetVec3(pub Vec3);

#[derive(Component, Clone)]
pub struct AnimalAnimations {
    pub graph: Handle<AnimationGraph>,
    pub nodes: HashMap<AnimalState, AnimationNodeIndex>,
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnimalState {
    Idle,
    Rest,
    Walk,
    Run,
    Fly,
    Sting,
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum AnimalKind {
    Butterfly,
}
