use std::f32::consts::FRAC_PI_4;

use crate::collision::{
    components::{ModelCollider, Shape},
    utils::add_collider,
};

use super::components::*;
use bevy::prelude::*;

pub fn spawn_car(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Transform,
) -> Entity {
    let car = commands
        .spawn((
            Car {
                direction: 0.0,
                actual: 0.0,
                target: 0.0,
                velocity: 0.0,
            },
            position,
        ))
        .id();

    commands.entity(car).with_children(|parent| {
        parent.spawn((
            CarVisual {
                roll: 0.0,
                equilibrium: 1.0,
                last_speed: 0.0,
            },
            Transform::IDENTITY,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/car/car.glb"))),
        ));
    });

    let offset = Transform {
        translation: Vec3 {
            x: 1.71217,
            y: 0.667857,
            z: 2.09069,
        },
        rotation: Quat::from_rotation_z(-1.570796),
        ..Default::default()
    };

    commands.entity(car).with_children(|parent| {
        parent.spawn((
            Wheel {
                position: WheelPosition::FrontLeft,
                current: 0.0,
                spin: 0.0,
                offset,
            },
            offset,
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/car/wheel.glb")),
            ),
        ));
    });

    let offset = Transform {
        translation: Vec3 {
            x: -1.71217,
            y: 0.667857,
            z: 2.09069,
        },
        rotation: Quat::from_rotation_z(1.570796),
        ..Default::default()
    };

    commands.entity(car).with_children(|parent| {
        parent.spawn((
            Wheel {
                position: WheelPosition::FrontRight,
                current: 0.0,
                spin: 0.0,
                offset,
            },
            offset,
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/car/wheel.glb")),
            ),
        ));
    });

    let offset = Transform {
        translation: Vec3 {
            x: 1.71217,
            y: 0.667857,
            z: -2.58032,
        },
        rotation: Quat::from_rotation_z(-1.570796),
        ..Default::default()
    };

    commands.entity(car).with_children(|parent| {
        parent.spawn((
            Wheel {
                position: WheelPosition::RearLeft,
                current: 0.0,
                spin: 0.0,

                offset,
            },
            offset,
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/car/wheel.glb")),
            ),
        ));
    });

    let offset = Transform {
        translation: Vec3 {
            x: -1.71217,
            y: 0.667857,
            z: -2.58032,
        },
        rotation: Quat::from_rotation_z(1.570796),
        ..Default::default()
    };

    commands.entity(car).with_children(|parent| {
        parent.spawn((
            Wheel {
                position: WheelPosition::RearRight,
                current: 0.0,
                spin: 0.0,

                offset,
            },
            offset,
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/car/wheel.glb")),
            ),
        ));
    });

    let collider = ModelCollider {
        position: Vec3::new(0.0, 2.0680195, -0.15),
        rotation: Vec3::new(0.0, 0.0, FRAC_PI_4),
        shape: Shape::Box(Vec3::new(3.926, 2.7388, 7.4441)),
        ..default()
    };

    add_collider(commands, car, collider);

    car
}
