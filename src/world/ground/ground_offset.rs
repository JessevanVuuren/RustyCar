use std::time::Instant;

use crate::world::{
    components::{
        Comp, Land, LandConfig, Model, Object, Offset, Placement, QUAD_POINTS, Range, Rotation,
        StaticWorld, TILE_SIZE, TilePos, TileType, TileWorld, Value,
    },
    ground::mesh_utils::{set_mesh_position, tile_mesh_positions},
    utils::range_from_surface,
};
use bevy::prelude::*;

pub fn ground_offset(
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d, With<Land>>,
    mut transforms: Query<&mut Transform, With<Object>>,
) {
    let mut calc = 0;

    for mut transform in transforms {
        let tile = TilePos::transform_to_tile(&transform);
        let ground = world.ground.get(&tile);

        if let Some((mesh, handle)) = tile_mesh_positions(&world, tile, &query, &meshes) {
            let height = avg_mesh_height(&mesh);
            transform.translation.y += height;
            calc += 1;
        }
    }

    println!("calc: {calc}");
}

fn avg_mesh_height(tile: &[[f32; 3]]) -> f32 {
    tile.iter().fold(0.0, |acc, coord| acc + coord[1]) / tile.len() as f32
}
