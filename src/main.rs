#![allow(unused)]
mod animal;
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
use rand::{SeedableRng, rngs::SmallRng};

use crate::{
    animal::AnimalPlugin,
    car::components::Car,
    world::{WorldPlugin, components::TileWorld},
    world_config::{grass_with_patches, large_grass_test, multiple_surface, test_world},
};

#[derive(Resource)]
pub struct Random {
    pub rng: SmallRng,
}

#[derive(Component)]
struct MainCamera {
    offset: Transform,
    current: Vec3,
}

const SEED: u64 = 1604;

fn main() {
    // let static_world = multiple_surface();
    // let static_world = test_world();
    // let static_world = large_grass_test();
    // let static_world = lots_of_patches();
    let static_world = grass_with_patches();
    // let static_world = multiple_surface();

    App::new()
        .init_resource::<TileWorld>()
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
        .add_systems(Startup, init_rng)
        // .add_plugins(CarPlugin)
        .add_plugins(WorldPlugin {
            static_world: static_world.clone(),
        })
        .add_plugins(AnimalPlugin {
            static_world: static_world.clone(),
        })
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, xyz_gismos)
        // .add_systems(Startup, setup_car)
        .add_systems(FixedUpdate, camera_follow)
        .run();
}

fn setup_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Transform::from_xyz(60.0, 0.0, 60.0);
    spawn_car(&mut commands, &asset_server, position);
}

fn init_rng(mut commands: Commands) {
    commands.insert_resource(Random {
        rng: SmallRng::seed_from_u64(SEED),
    });
}

fn xyz_gismos(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), RED);
    gizmos.line(Vec3::ZERO, Vec3::new(0.0, 0.0, 4.0), BLUE);
    gizmos.line(Vec3::ZERO, Vec3::new(0.0, 4.0, 0.0), GREEN);
}

fn setup_camera(mut commands: Commands) {
    // let focus = Vec3::new(0.0, 0.0, 0.0);
    // let offset = Transform::from_xyz(20.0, 30.0, 40.0).looking_at(focus, Vec3::Y);
    let focus = Vec3::new(6.0, 0.0, 6.0);
    let offset = Transform::from_xyz(23.0, 10.0, 23.0).looking_at(focus, Vec3::Y);
    // let focus = Vec3::new(15.0, 0.0, 15.0);
    // let offset = Transform::from_xyz(40.0, 20.0, 40.0).looking_at(focus, Vec3::Y);
    // let focus = Vec3::new(60.0, 0.0, 60.0);
    // let offset = Transform::from_xyz(90.0, 30.0, 80.0).looking_at(focus, Vec3::Y);

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.7),
            // color: Color::srgb(1.0, 1.0, 1.0),
            illuminance: 5_000.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.6, -0.7, -0.7), Vec3::Y),
    ));

    commands.insert_resource(ClearColor(Color::srgb(0.6, 0.8, 1.0)));

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
