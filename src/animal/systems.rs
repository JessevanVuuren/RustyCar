use crate::animal::components::{AnimalAnimations, AnimalState};
use bevy::prelude::*;
use std::time::Duration;

pub fn link_animal_animations(
    mut commands: Commands,
    hierarchy: Query<&ChildOf>,
    parents: Query<(&AnimalAnimations, &AnimalState)>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        for ancestor in hierarchy.iter_ancestors(entity) {
            if let Ok((config, default_anim)) = parents.get(ancestor) {
                let mut transitions = AnimationTransitions::new();
                let animation = *config.nodes.get(default_anim).unwrap();

                transitions
                    .play(&mut player, animation, Duration::ZERO)
                    .repeat();

                commands
                    .entity(entity)
                    .insert(AnimationGraphHandle(config.graph.clone()))
                    .insert(transitions);

                break;
            }
        }
    }
}

pub fn update_animal_animations(
    hierarchy: Query<&ChildOf>,
    parents: Query<(&AnimalState, &AnimalAnimations), Changed<AnimalState>>,
    mut animation_players: Query<(Entity, &mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (entity, mut player, mut transitions) in &mut animation_players {
        for ancestor in hierarchy.iter_ancestors(entity) {
            if let Ok((state, config)) = parents.get(ancestor) {
                if let Some(&target_node) = config.nodes.get(state) {
                    transitions
                        .play(&mut player, target_node, Duration::from_millis(250))
                        .repeat();
                }
                break;
            }
        }
    }
}
