use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Butterfly;

#[derive(Resource, Default)]
pub struct AnimalLibrary {
    pub butterfly: Option<AnimalAnimations>,
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
