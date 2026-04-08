use crate::{
    Random,
    animal::{
        components::{
            AnimalState, Butterfly, ButterflyMovement, ButterflyState, RestTimer, TargetFlower,
        },
        globals::{BUTTERFLY_SPEED, MIN_DIST},
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
                let distance = start.translation.distance(stop.translation);

                commands
                    .entity(entity)
                    .insert(TargetFlower(flower))
                    .insert(ButterflyMovement {
                        distance,
                        base_y: start.translation.y,
                    })
                    .insert(ButterflyState::Moving);
            }
        }
    }
}

pub fn animate_butterfly(
    time: Res<Time>,
    mut commands: Commands,
    mut random: ResMut<Random>,
    flowers: Query<&Transform, (With<Flower>, Without<Butterfly>)>,
    butterflies: Query<
        (Entity, &mut Transform, &TargetFlower, &ButterflyMovement),
        With<Butterfly>,
    >,
) {
    for (entity, mut transform, target, movement) in butterflies {
        if let Ok(flower) = flowers.get(target.0) {
            let current_distance = transform.translation.distance(flower.translation);

            if current_distance >= MIN_DIST {
                let dir = (flower.translation - transform.translation).normalize();
                transform.translation += dir * BUTTERFLY_SPEED * time.delta_secs();

                // let curr = current_distance;
                // let end = movement.distance;

                // let t = ((curr - MIN_DIST) / (end - MIN_DIST)).clamp(0.0, 1.0);
                // let amplitude = 4.0 * t * (1.0 - t);
                // println!("{amplitude}");
                // transform.translation.y = movement.base_y + amplitude * 10.0;

                transform.look_at(flower.translation, Vec3::Y);
            } else {
                let rest = random.rng.random_range(1.0..10.0);
                let timer = Timer::from_seconds(rest, TimerMode::Once);

                commands
                    .entity(entity)
                    .remove::<TargetFlower>()
                    .insert(RestTimer(timer))
                    .insert(AnimalState::Idle)
                    .insert(ButterflyState::Resting);
            }
        }
    }
}
