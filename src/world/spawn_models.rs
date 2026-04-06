use crate::world::{
    components::{
        BASE_ASSET, Comp, Dirt, Fence, Flower, GroundId, Ground, GroundConfig, Log, Model, Mushroom,
        Object, Offset, Placement, Range, Rock, Rotation, StaticWorld, TileType, TileWorld, Tree,
        Value,
    },
    ground::ground::ground_plane,
    tile_pos::TilePos,
    utils::{
        add_component_to_entity, apply_transformations, model_path, range_from_surfaces,
        spawn_object, tiles_range_from_placement,
    },
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{f32::consts::FRAC_PI_2, iter, slice::from_ref};

pub fn spawn_models(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    for block in &static_world.blocks {
        if let TileType::Models(models) = &block.tiletype {
            for object in models {
                let path = model_path(&mut rng, &object);

                let range = range_from_surfaces(&block.surface).collect();
                let tiles = tiles_range_from_placement(&mut rng, &object.placement.amount, range);

                for tile in tiles {
                    let mut transform = tile.to_world_transform();
                    apply_transformations(&mut rng, &mut transform, &object.placement);

                    let id = spawn_object(&transform, &path, &mut commands, &assets);
                    add_component_to_entity(&mut commands, &object.comp, id);

                    world.models.entry(tile).or_default().push(id)
                }
            }
        }
    }
}
