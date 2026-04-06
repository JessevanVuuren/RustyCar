use crate::world::{
    components::{
        BASE_ASSET, Comp, Dirt, Fence, Flower, GroundId, Ground, GroundConfig, Log, Model, Mushroom,
        Object, Offset, Placement, Range, Rock, Rotation, StaticWorld, TileType, TileWorld, Tree,
        Value,
    },
    ground::ground::ground_plane,
    tile_pos::TilePos,
    utils::range_from_surfaces,
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{f32::consts::FRAC_PI_2, iter, slice::from_ref};

pub fn spawn_ground(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    for (layer, block) in static_world.blocks.iter().enumerate() {
        if let TileType::Ground(ground) = &block.tiletype {
            let range = range_from_surfaces(&block.surface);

            for tile in range {
                let mut pos = tile.to_world_transform();
                let grass = ground_plane(&mut rng, pos.translation, &ground);

                let id = spawn_grass(grass, &pos, &mut commands, &mut meshes, &mut materials);

                world.ground.insert(
                    tile,
                    GroundId {
                        entity: id,
                        id: layer + 1,
                    },
                );
            }
        }
    }
}

fn spawn_grass(
    grass: Mesh,
    transform: &Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    commands
        .spawn((
            Ground,
            Mesh3d(meshes.add(grass)),
            MeshMaterial3d(materials.add(Color::WHITE)),
            transform.clone(),
        ))
        .id()
}
