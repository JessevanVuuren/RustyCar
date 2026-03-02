mod car;

use bevy::prelude::*;

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use car::{CarPlugin, spawn::spawn_car};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CarPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, (setup_world, setup))
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50., 25., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(10.0, 10.0, 10.0)),
        PanOrbitCamera::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d {
            normal: Dir3::Y,
            half_size: Vec2 { x: 50., y: 50. },
        })),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::IDENTITY,
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_car(&mut commands, &asset_server);
}
