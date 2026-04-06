use crate::world::{
    components::{Ground, Object, TileWorld},
    ground::mesh_utils::tile_mesh_positions,
    tile_pos::TilePos,
};
use bevy::prelude::*;

pub fn ground_offset(
    world: Res<TileWorld>,
    meshes: Res<Assets<Mesh>>,
    query: Query<&Mesh3d, With<Ground>>,
    transforms: Query<&mut Transform, With<Object>>,
) {
    for mut transform in transforms {
        let tile = TilePos::transform_to_tile(&transform);

        if let Some((mesh, handle)) = tile_mesh_positions(&world, tile, &query, &meshes) {
            let height = avg_mesh_height(&mesh);
            transform.translation.y += height;
        }
    }
}

fn avg_mesh_height(tile: &[[f32; 3]]) -> f32 {
    tile.iter().fold(0.0, |acc, coord| acc + coord[1]) / tile.len() as f32
}
