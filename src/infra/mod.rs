use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use crate::infra::{components::Infra, spawn::spawn_infras};

pub mod components;
pub mod spawn;

pub struct InfraPlugin;

impl Plugin for InfraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

const FENCE_WIDTH: f32 = 1.9621;

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = SmallRng::seed_from_u64(1604);
    let mut infra = Vec::new();

    for i in 0..20 {
        let fence = rng.random_range(1..5);
        infra.push(Infra {
            name: "Fence".into(),
            path: format!("fence_{fence}"),
            transform: Transform::from_xyz(i as f32 * FENCE_WIDTH, 0.0, 5.0),
        })
    }

    spawn_infras(&mut commands, &asset_server, infra);
}
