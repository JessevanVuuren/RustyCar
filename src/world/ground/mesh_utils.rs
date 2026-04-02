use bevy::{mesh::VertexAttributeValues, prelude::*};

use crate::world::components::{Land, TilePos, TileWorld};

pub fn tile_mesh_positions(
    world: &TileWorld,
    tile: TilePos,
    query: &Query<&Mesh3d, With<Land>>,
    meshes: &Assets<Mesh>,
) -> Option<(Vec<[f32; 3]>, Handle<Mesh>)> {
    if let Some(handle) = mesh_on_tile(&world, tile, &query) {
        let mesh = meshes.get(&handle)?;
        return match mesh.attribute(Mesh::ATTRIBUTE_POSITION)? {
            VertexAttributeValues::Float32x3(pos) => Some((pos.clone(), handle)),
            _ => None,
        };
    }

    None
}

pub fn tile_mesh_colors(
    world: &TileWorld,
    tile: TilePos,
    query: &Query<&Mesh3d, With<Land>>,
    meshes: &Assets<Mesh>,
) -> Option<(Vec<[f32; 4]>, Handle<Mesh>)> {
    if let Some(handle) = mesh_on_tile(&world, tile, &query) {
        let mesh = meshes.get(&handle)?;
        return match mesh.attribute(Mesh::ATTRIBUTE_COLOR)? {
            VertexAttributeValues::Float32x4(color) => Some((color.clone(), handle)),
            _ => None,
        };
    }

    None
}

pub fn mesh_on_tile(
    world: &TileWorld,
    tile: TilePos,
    query: &Query<&Mesh3d, With<Land>>,
) -> Option<Handle<Mesh>> {
    if let Some(t) = world.ground.get(&tile) {
        if let Ok(mesh3d) = query.get(t.entity.entity()) {
            return Some(mesh3d.0.clone());
        }
    }

    None
}

pub fn set_mesh_position(
    positions: &Vec<[f32; 3]>,
    handle: &Handle<Mesh>,
    meshes: &mut Assets<Mesh>,
) {
    if let Some(mesh) = meshes.get_mut(handle) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.compute_flat_normals();
    }
}

pub fn set_mesh_colors(colors: &Vec<[f32; 4]>, handle: &Handle<Mesh>, meshes: &mut Assets<Mesh>) {
    if let Some(mesh) = meshes.get_mut(handle) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.clone());
        mesh.compute_flat_normals();
    }
}
