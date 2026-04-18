use std::{ops::Neg, time::Duration};

use crate::{
    Random,
    animal::{
        behaviors::{NaturalFlyPath, SwirlingPath},
        components::{
            AnimalState, Butterfly, FlowerBed, FreeFly, RestTimer, Swirling, TargetEntity,
            TargetVec3,
        },
        globals::FREE_FLY_HEIGHT,
    },
    extra::{
        math::{abs_sin, arc, flat, s_curve},
        utils::{comma_print, debug_sphere},
    },
    world::components::Flower,
};
use bevy::prelude::*;
use rand::{RngExt, seq::IteratorRandom};

pub fn butterfly_swirl(
    time: Res<Time>,
    mut commands: Commands,
    mut last_pos: Local<Vec3>,
    mut random: ResMut<Random>,
    butterflies: Query<
        (Entity, &mut Transform, &mut SwirlingPath),
        (With<Butterfly>, With<Swirling>),
    >,
) {
    for (entity, mut transform, mut swirling) in butterflies {
        let pos = swirling.sample(time.elapsed_secs());
        let target = swirling.look_at(pos);

        transform.translation = pos;
        transform.look_at(target, Vec3::Y);

        swirling.last_pos(pos);
    }
}

pub fn butterfly_assign_freefly_target(
    mut commands: Commands,
    mut random: ResMut<Random>,
    butterflies: Query<(Entity, &Transform, &FreeFly), (With<Butterfly>, Without<TargetVec3>)>,
) {
    for (entity, start, freefly) in butterflies {
        if let Some(tile) = freefly.0.iter().choose(&mut random.rng) {
            let mut stop = tile.to_random_world_transform(&mut random.rng);
            stop.translation.y = FREE_FLY_HEIGHT;

            let movement =
                NaturalFlyPath::random(&mut random.rng, start.translation, stop.translation);

            commands
                .entity(entity)
                .insert(movement)
                .insert(AnimalState::Fly)
                .insert(TargetVec3(stop.translation));
        }
    }
}

pub fn butterfly_assign_flowerbed_flower(
    mut commands: Commands,
    mut random: ResMut<Random>,
    flowers: Query<(Entity, &Transform, &FlowerBed), With<Flower>>,
    butterflies: Query<
        (Entity, &Transform, &AnimalState, &FlowerBed),
        (With<Butterfly>, Without<TargetEntity>),
    >,
) {
    for (entity, start, state, butterfly_id) in butterflies {
        if matches!(state, AnimalState::Idle) {
            let flowers = flowers.iter().filter(|(_, _, id)| id.0 == butterfly_id.0);

            if let Some((flower, stop, id)) = flowers.choose(&mut random.rng) {
                let movement =
                    NaturalFlyPath::random(&mut random.rng, start.translation, stop.translation);

                commands
                    .entity(entity)
                    .insert(movement)
                    .insert(AnimalState::Fly)
                    .insert(TargetEntity(flower));
            }
        }
    }
}

pub fn butterfly_finish_freefly(
    mut commands: Commands,
    query: Query<(Entity, &NaturalFlyPath), (With<FreeFly>, With<TargetVec3>)>,
) {
    for (entity, path) in &query {
        if path.is_finished() {
            commands.entity(entity).remove::<TargetVec3>();
        }
    }
}

pub fn butterfly_finish_flowerbed(
    mut commands: Commands,
    query: Query<(Entity, &NaturalFlyPath), (With<Butterfly>, With<FlowerBed>, With<TargetEntity>)>,
) {
    for (entity, path) in &query {
        if path.is_finished() {
            commands
                .entity(entity)
                .remove::<TargetEntity>()
                .insert(RestTimer(path.rest_timer()))
                .insert(AnimalState::Rest);
        }
    }
}
