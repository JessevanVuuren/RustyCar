mod car;
mod extra;
mod world;
mod world_config;

use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
};

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use car::{CarPlugin, spawn::spawn_car};

use crate::{
    car::components::Car,
    world::{
        WorldPlugin,
        components::{GrassConfig, Noise},
        grass::grass_plane,
    },
    world_config::{grass_test, test_world},
};

fn main() {
    let static_world = test_world();
    // let static_world = grass_test();

    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: {
                    FpsOverlayConfig {
                        frame_time_graph_config: FrameTimeGraphConfig {
                            enabled: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
            },
        ))
        .add_plugins(CarPlugin)
        // .add_plugins(WorldPlugin { static_world })
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_grass)
        .add_systems(Update, xyz_gismos)
        // .add_systems(Startup, setup_car)
        .add_systems(FixedUpdate, camera_follow)
        .run();
}

fn setup_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_car(&mut commands, &asset_server);
}

#[derive(Component)]
struct MainCamera {
    offset: Transform,
    current: Vec3,
}

fn spawn_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grass = grass_plane(
        Vec3::ZERO,
        4,
        4.0,
        GrassConfig {
            color: {
                Noise {
                    scale: 40.0,
                    val1: Color::linear_rgb(0.0, 0.69, 0.22),
                    val2: Color::linear_rgb(0.624, 1.0, 0.745),
                }
            },
            height: {
                Noise {
                    scale: 0.1,
                    val1: 0.0,
                    val2: 0.05,
                }
            },
            colors: vec![
                Color::linear_rgb(1.0, 1.0, 1.0),
                Color::linear_rgb(0.0, 0.0, 0.0),
                // Color::linear_rgb(0.125, 0.545, 0.227),
                // Color::linear_rgb(0.145, 0.635, 0.267),
                // Color::linear_rgb(0.176, 0.776, 0.325),
                // Color::linear_rgb(0.29, 0.839, 0.427),
            ],
        },
    );

    commands.spawn((
        Mesh3d(meshes.add(grass)),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::IDENTITY,
    ));
}

fn xyz_gismos(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), RED);
    gizmos.line(Vec3::ZERO, Vec3::new(0.0, 0.0, 4.0), BLUE);
    gizmos.line(Vec3::ZERO, Vec3::new(0.0, 4.0, 0.0), GREEN);
}

fn setup_camera(mut commands: Commands) {
    let focus = Vec3::new(0.0, 0.0, 0.0);
    let offset = Transform::from_xyz(30.0, 30.0, 40.0).looking_at(focus, Vec3::Y);

    // let focus = Vec3::new(60.0, 0.0, 60.0);
    // let offset = Transform::from_xyz(90.0, 30.0, 80.0).looking_at(focus, Vec3::Y);

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // commands.spawn((
    //     DirectionalLight {
    //         // color: Color::srgb(1., 0.95, 0.7),
    //         color: Color::srgb(1.0, 1.0, 1.0),
    //         illuminance: 2_500.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-1.0, -0.7, -0.6), Vec3::Y),
    // ));
    // commands.insert_resource(ClearColor(Color::srgb(0.6, 0.8, 1.0)));

    commands.spawn((offset, PanOrbitCamera { focus, ..default() }));

    // commands.spawn((
    //     MainCamera {
    //         offset,
    //         current: offset.translation,
    //     },
    //     Projection::Orthographic(OrthographicProjection {
    //         scaling_mode: ScalingMode::FixedHorizontal {
    //             viewport_width: 70.0,
    //         },
    //         ..OrthographicProjection::default_3d()
    //     }),
    //     Camera3d::default(),
    //     offset,
    // ));
}

const CAMERA_SPEED: f32 = 2.0;

fn camera_follow(
    mut transforms: ParamSet<(
        Single<&Transform, With<Car>>,
        Single<(&mut Transform, &mut MainCamera)>,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let car_translation = transforms.p0().translation;
    let (mut transform, mut camera) = transforms.p1().into_inner();

    let target = car_translation + camera.offset.translation;
    camera.current = camera.current.lerp(target, dt * CAMERA_SPEED);

    transform.translation = camera.current;
}
