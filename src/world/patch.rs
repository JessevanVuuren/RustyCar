use crate::world::components::{
    Comp, GrassConfig, Model, Offset, Placement, Range, Rotation, StaticWorld, TilePos, TileType,
    TileWorld, Value,
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

pub fn patch_ground(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}
