use std::time::Duration;

use crate::{
    Random,
    animal::{
        animals::components::{ButterflyPath, ButterflyState},
        components::{AnimalState, Butterfly, RestTimer, TargetFlower},
        globals::MIN_DIST,
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

pub fn butterfly_assign_flower(
    mut commands: Commands,
    mut random: ResMut<Random>,
    flowers: Query<(Entity, &Transform), With<Flower>>,
    butterflies: Query<(Entity, &Transform, &ButterflyState), With<Butterfly>>,
) {
    for (entity, start, state) in butterflies {
        if matches!(state, ButterflyState::Searching) {
            if let Some((flower, stop)) = flowers.iter().choose(&mut random.rng) {
                let start = start.translation;
                let stop = stop.translation;
                let movement = ButterflyPath::max_values(&mut random.rng, start, stop);

                commands
                    .entity(entity)
                    .insert(movement)
                    .insert(TargetFlower(flower))
                    .insert(ButterflyState::Moving);
            }
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
    paths: Query<(&ButterflyPath, &ButterflyState), Changed<ButterflyState>>,
) {
    for (path, state) in paths {
        if matches!(state, ButterflyState::Moving) {
            *local = time.elapsed();

            for i in 0..100 {
                let pos = path.sample(i as f32 / 100 as f32);
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

pub fn animate_butterfly(
    time: Res<Time>,
    mut commands: Commands,
    mut random: ResMut<Random>,
    butterflies: Query<
        (Entity, &mut Transform, &mut ButterflyPath),
        (With<Butterfly>, With<TargetFlower>),
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
                .remove::<TargetFlower>()
                .insert(RestTimer(path.rest_timer()))
                .insert(AnimalState::Idle)
                .insert(ButterflyState::Resting);
        }
    }
}
