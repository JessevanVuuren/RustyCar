use super::components::*;
use bevy::prelude::*;

pub fn spawn_car(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let car = commands
        .spawn((
            Car {
                direction: 0.0,
                actual: 0.0,
                target: 0.0,
                velocity: 0.0,
            },
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/car.glb"))),
            Transform::default(),
        ))
        .id();

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
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wheel.glb"))),
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
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wheel.glb"))),
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
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wheel.glb"))),
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
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wheel.glb"))),
        ));
    });

    car
}
