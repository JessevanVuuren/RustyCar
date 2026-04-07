use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationToPlay {
    pub graph_handle: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}