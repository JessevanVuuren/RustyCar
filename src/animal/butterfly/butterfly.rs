use std::time::Duration;

use crate::{
    Random,
    animal::{
        butterfly::components::{ButterflyState, NaturalFlyPath},
        components::{
            AnimalState, Butterfly, FlowerBed, FreeFly, RestTimer, TargetEntity, TargetVec3,
        },
        globals::FREE_FLY_HEIGHT,
    },
    extra::{
        math::{arc, flat, normalized_sin, s_curve},
        utils::{comma_print, debug_sphere},
    },
    world::components::Flower,
};
use bevy::prelude::*;
use rand::{RngExt, seq::IteratorRandom};

pub fn update_rest_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut RestTimer), With<ButterflyState>>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.is_finished() {
            commands
                .entity(entity)
                .remove::<RestTimer>()
                .insert(ButterflyState::Searching)
                .insert(AnimalState::Fly);
        }
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
                .insert(TargetVec3(stop.translation));
        }
    }
}

pub fn butterfly_assign_flowerbed_flower(
    mut commands: Commands,
    mut random: ResMut<Random>,
    flowers: Query<(Entity, &Transform, &FlowerBed), With<Flower>>,
    butterflies: Query<
        (Entity, &Transform, &ButterflyState, &FlowerBed),
        (With<Butterfly>, Without<TargetEntity>),
    >,
) {
    for (entity, start, state, butterfly_id) in butterflies {
        if matches!(state, ButterflyState::Searching) {
            let flowers = flowers.iter().filter(|(_, _, id)| id.0 == butterfly_id.0);

            if let Some((flower, stop, id)) = flowers.choose(&mut random.rng) {
                let movement =
                    NaturalFlyPath::random(&mut random.rng, start.translation, stop.translation);

                commands
                    .entity(entity)
                    .insert(movement)
                    .insert(TargetEntity(flower))
                    .insert(ButterflyState::Moving);
            }
        }
    }
}

pub fn butterfly_animate_flowerbed(
    time: Res<Time>,
    mut commands: Commands,
    butterflies: Query<
        (Entity, &mut Transform, &mut NaturalFlyPath),
        (With<Butterfly>, With<FlowerBed>, With<TargetEntity>),
    >,
) {
    for (entity, mut transform, mut path) in butterflies {
        path.step(time.delta_secs());

        if !path.is_finished() {
            let pos = path.position();
            let target = path.look_at(pos);

            transform.translation = pos;
            transform.look_at(target, Vec3::Y);
        } else {
            commands
                .entity(entity)
                .remove::<TargetEntity>()
                .insert(RestTimer(path.rest_timer()))
                .insert(AnimalState::Idle)
                .insert(ButterflyState::Resting);
        }
    }
}

pub fn butterfly_animate_freefly(
    time: Res<Time>,
    mut commands: Commands,
    butterflies: Query<
        (Entity, &mut Transform, &mut NaturalFlyPath),
        (With<Butterfly>, With<FreeFly>, With<TargetVec3>),
    >,
) {
    for (entity, mut transform, mut path) in butterflies {
        path.step(time.delta_secs());

        if !path.is_finished() {
            let pos = path.position();
            let target = path.look_at(pos);

            transform.translation = pos;
            transform.look_at(target, Vec3::Y);
        } else {
            commands.entity(entity).remove::<TargetVec3>();
        }
    }
}

pub fn debug_butterfly_path(
    time: Res<Time>,
    mut commands: Commands,
    mut local: Local<Duration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut dots: Local<Vec<Entity>>,
    paths: Query<(&NaturalFlyPath, &ButterflyState), Changed<ButterflyState>>,
) {
    for (path, state) in paths {
        if matches!(state, ButterflyState::Moving) {
            *local = time.elapsed();

            for i in 0..500 {
                let pos = path.sample(i as f32 / 500 as f32);
                dots.push(debug_sphere(
                    pos,
                    0.05,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                ));
            }
        }
        if matches!(state, ButterflyState::Resting) {
            let time = time.elapsed() - *local;
            let speed = path.units_per_sec(time);
            dots.clear();

            println!("time: {:?}, units/sec: {}", time, speed);
        }
    }
}
