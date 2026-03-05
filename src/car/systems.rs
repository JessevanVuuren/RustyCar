use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::car::components::{Car, CarVisual, Wheel, WheelPosition};

const DRIVE_SPEED: f32 = 1.0;

const MAX_SPEED_FORWARD: f32 = 50.0;
const MAX_SPEED_BACKWARD: f32 = 10.0;

const SPEED_DAMP: f32 = 0.99;
const STEER_DAMP: f32 = 0.14;
const STEER_ANGLE: f32 = 0.5;

const PITCH_GROW: f32 = 1.0;
const PITCH_DECAY: f32 = 0.6;
const PITCH_MAX: f32 = 0.15;

const ROLL_MAX: f32 = 0.15;
const ROLL_SPEED: f32 = 3.0;

pub fn car_input(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Car>) {
    for mut car in query.iter_mut() {
        if keyboard.pressed(KeyCode::KeyA) {
            car.target = -STEER_ANGLE + car.actual;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            car.target = STEER_ANGLE + car.actual;
        }

        if keyboard.pressed(KeyCode::KeyW) {
            car.velocity += DRIVE_SPEED;
            car.direction = 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            car.velocity -= DRIVE_SPEED;
            car.direction = -1.0;
        }
    }
}

pub fn car_tilt(
    mut wheels: Query<(&mut Transform, &mut CarVisual, &ChildOf)>,
    cars: Query<&Car>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut visual, parent) in wheels.iter_mut() {
        let car = cars.get(parent.get()).unwrap();

        let norm = car.velocity / MAX_SPEED_FORWARD;

        if car.velocity == MAX_SPEED_FORWARD || car.velocity < visual.last_speed {
            visual.equilibrium -= PITCH_DECAY * dt;
        } else {
            visual.equilibrium += PITCH_GROW * dt;
        }
        visual.equilibrium = visual.equilibrium.clamp(0.0, 1.0);
        visual.last_speed = car.velocity;

        let target_tilt = (-norm * PITCH_MAX) * visual.equilibrium;

        let lean_angle = (car.actual - car.target) * ROLL_SPEED * norm;
        visual.roll = visual.roll.lerp(lean_angle, dt).clamp(-ROLL_MAX, ROLL_MAX);

        transform.rotation =
            Quat::from_rotation_x(target_tilt) * Quat::from_rotation_z(visual.roll);
    }
}

pub fn car_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Car)>) {
    let dt = time.delta_secs();

    for (mut transform, mut car) in query.iter_mut() {
        let actual_forward = Vec3::new(f32::cos(car.actual), 0.0, f32::sin(car.actual));
        let offset_forward = actual_forward + transform.translation;

        let direction = offset_forward - transform.translation;
        transform.translation += direction * car.velocity * dt;

        transform.rotation = Quat::from_rotation_y(-car.actual + 1.5707963268);

        let diff = car.target - car.actual;
        let norm_steer = (car.velocity.abs() / MAX_SPEED_FORWARD).clamp(0.0, 1.0);

        car.actual += diff * norm_steer * STEER_DAMP;

        car.velocity *= SPEED_DAMP;
        car.velocity = car.velocity.clamp(-MAX_SPEED_BACKWARD, MAX_SPEED_FORWARD);
    }
}

pub fn wheel_steering(
    time: Res<Time>,
    mut wheels: Query<(&mut Transform, &mut Wheel, &ChildOf)>,
    cars: Query<&Car>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut wheel, parent) in wheels.iter_mut() {
        let car = cars.get(parent.get()).unwrap();

        wheel.spin += car.velocity * dt;

        if matches!(
            wheel.position,
            WheelPosition::FrontLeft | WheelPosition::FrontRight
        ) {
            let diff = car.target - car.actual;
            wheel.current = wheel.current.lerp(-diff, dt * 8.);

            transform.rotation = Quat::from_rotation_y(wheel.current)
                * Quat::from_rotation_x(wheel.spin)
                * wheel.offset.rotation;
        } else {
            transform.rotation = Quat::from_rotation_x(wheel.spin) * wheel.offset.rotation;
        }
    }
}
