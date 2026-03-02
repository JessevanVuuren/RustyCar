use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::car::components::{Car, Wheel, WheelPosition};

const DRIVE_SPEED: f32 = 1.0;
const CAR_DAMP: f32 = 0.99;

const STEER_ANGLE: f32 = 0.5;
const STEER_SPEED: f32 = 0.1;

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
        }
        if keyboard.pressed(KeyCode::KeyS) {
            car.velocity -= DRIVE_SPEED;
        }
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

        let steer_speed = dt * STEER_SPEED * car.velocity.length();
        car.actual = car.actual.lerp(car.target, steer_speed);

        car.velocity *= CAR_DAMP;
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

        if matches!(
            wheel.position,
            WheelPosition::FrontLeft | WheelPosition::FrontRight
        ) {
            let diff = car.target - car.actual;
            wheel.current = wheel.current.lerp(-diff, dt * 8.);

            transform.rotation = Quat::from_rotation_y(wheel.current) * wheel.offset.rotation;
        }
    }
}
