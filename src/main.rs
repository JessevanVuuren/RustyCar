mod building;
mod car;
mod infra;
mod nature;

use bevy::{
    camera::ScalingMode,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
};

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use car::{CarPlugin, spawn::spawn_car};

use crate::{
    building::BuildingPlugin, car::components::Car, infra::InfraPlugin, nature::EnvironmentPlugin,
};

fn main() {
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
        .add_plugins(InfraPlugin)
        .add_plugins(BuildingPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_default)
        .add_systems(Startup, setup_car)
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
fn setup_default(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(500., 250., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let offset = Transform::from_xyz(30.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y);

    // commands.spawn((
    //     MainCamera {
    //         offset,
    //         current: offset.translation,
    //     },
    //     offset,
    //     PanOrbitCamera::default(),
    // ));

    commands.spawn((
        MainCamera {
            offset,
            current: offset.translation,
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: 70.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Camera3d::default(),
        offset,
    ));
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
