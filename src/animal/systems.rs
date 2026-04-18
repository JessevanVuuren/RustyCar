use crate::animal::{
    behaviors::NaturalFlyPath,
    components::{AnimalAnimations, AnimalState, RestTimer},
};
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

pub fn animal_animate_natural_fly_path(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<(Entity, &mut Transform, &mut NaturalFlyPath)>,
) {
    for (entity, mut transform, mut path) in query {
        path.step(time.delta_secs());

        if !path.is_finished() {
            let pos = path.position();
            let target = path.look_at(pos);

            transform.translation = pos;
            transform.look_at(target, Vec3::Y);
        }
    }
}

pub fn update_rest_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut RestTimer), With<AnimalState>>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.is_finished() {
            commands
                .entity(entity)
                .remove::<RestTimer>()
                .insert(AnimalState::Idle);
        }
    }
}
