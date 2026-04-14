use bevy::prelude::*;
use std::collections::HashMap;

use crate::extra::components::Range;

#[derive(Clone, Debug)]
pub struct AnimalModel {
    pub amount: i32,
    pub path: String,
    pub kind: AnimalKind,
    pub offset: Transform,
    pub variation: f32,
    pub range: Range<i32>,
    pub animations: Vec<AnimalState>,
}

#[derive(Component)]
pub struct Butterfly;

#[derive(Component)]
pub struct FlowerBed(pub u8);

#[derive(Resource, Default)]
pub struct AnimalLibrary {
    pub animals: HashMap<AnimalKind, AnimalAnimations>,
}

#[derive(Component)]
pub struct RestTimer(pub Timer);

#[derive(Component)]
pub struct TargetFlower(pub Entity);

#[derive(Component, Clone)]
pub struct AnimalAnimations {
    pub graph: Handle<AnimationGraph>,
    pub nodes: HashMap<AnimalState, AnimationNodeIndex>,
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnimalState {
    Idle,
    Walk,
    Run,
    Fly,
    Sting,
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum AnimalKind {
    Butterfly,
}
