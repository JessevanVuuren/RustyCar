use crate::{collision::utils::add_collider, world::{
    components::{StaticWorld, TileType, TileWorld},
    utils::{
        add_component_to_entity, apply_transformations, model_path, range_from_surfaces,
        spawn_object, tiles_range_from_placement,
    },
}};
use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

pub fn spawn_models(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    for block in &static_world.blocks {
        if let TileType::Models(models) = &block.tiletype {
            for object in models {
                let path = model_path(&mut rng, &object.path, &object.range);

                let range = range_from_surfaces(&block.surface).collect();
                let tiles = tiles_range_from_placement(&mut rng, &object.placement.amount, range);

                for tile in tiles {
                    let mut transform = tile.to_world_transform();
                    apply_transformations(&mut rng, &mut transform, &object.placement);

                    let id = spawn_object(&transform, &path, &mut commands, &assets);
                    add_component_to_entity(&mut commands, &object.comp, id);

                    if let Some(collider) = &object.collider {
                        add_collider(&mut commands, id, collider.clone());
                    }

                    world.models.entry(tile).or_default().push(id)
                }
            }
        }
    }
}
