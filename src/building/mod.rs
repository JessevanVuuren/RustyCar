use bevy::prelude::*;

use crate::building::{components::Building, spawn::spawn_buildings};

pub mod components;
pub mod spawn;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let buildings = vec![Building {
        name: "Well".into(),
        path: "well".into(),
        transform: Transform::from_xyz(10., 0.0, 0.0).with_scale(Vec3::new(2.0, 2.0, 2.0)),
    }];

    spawn_buildings(&mut commands, &asset_server, buildings);
}
