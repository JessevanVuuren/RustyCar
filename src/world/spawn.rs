use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng, seq::IndexedRandom};

use crate::world::{
    components::{Dirt, Fence, Flower, Ground, TilePos, TileWorld},
    utils::{DOWN, LEFT, RIGHT, UP, transform_random_direction, transform_with_direction},
};

pub fn spawn_object<T: Component>(
    object: T,
    transform: &Transform,
    path: &str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            object,
            transform.clone(),
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.to_string()))),
        ))
        .id()
}

pub fn generate_flowers(
    mut commands: Commands,
    mut world: ResMut<TileWorld>,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Ground>>,
) {
    let amount = 100;
    let mut rng = SmallRng::seed_from_u64(1604);

    let grounds: Vec<&Transform> = query.iter().collect();

    for _ in 0..amount {
        if let Some(transform) = grounds.choose(&mut rng) {
            let tile = TilePos::transform_to_tile(transform);
            let pos = tile.to_random_world_transform(&mut rng);

            let flower_index = rng.random_range(1..9);
            let path = format!("models/nature/flower_{}.glb", flower_index);

            let flower_entity = spawn_object(Flower, &pos, &path, &mut commands, &asset_server);

            world
                .object
                .entry(tile)
                .or_insert_with(Vec::new)
                .push(flower_entity);
        }
    }
}

pub fn generate_ground(
    mut commands: Commands,
    mut world: ResMut<TileWorld>,
    asset_server: Res<AssetServer>,
) {
    let size = 10;
    let half = size / 2;

    let mut rng = SmallRng::seed_from_u64(1604);

    for x in 0..size {
        for z in 0..size {
            let tile_pos = TilePos { x, z } - half;
            let mut tile = tile_pos.to_world_transform();
            transform_random_direction(&mut rng, &mut tile);

            let entity = if z < 3 || x < 3 {
                spawn_object(
                    Dirt,
                    &tile,
                    "models/ground/dirt.glb",
                    &mut commands,
                    &asset_server,
                )
            } else {
                spawn_object(
                    Ground,
                    &tile,
                    "models/ground/grass.glb",
                    &mut commands,
                    &asset_server,
                )
            };
            world.ground.insert(tile_pos, entity);
        }
    }
}

pub fn generate_fencing(
    mut commands: Commands,
    mut world: ResMut<TileWorld>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    let size = 10;
    let half = size / 2;

    for x in 0..size {
        for z in 0..size {
            let tile_pos = TilePos { x, z } - half;

            let mut tile = tile_pos.to_world_transform();

            let mut fence_model = "".into();
            let fence_index = rng.random_range(1..3);

            if x == 0 && z == 0 {
                fence_model = format!("fence_corner_{fence_index}");
                transform_with_direction(DOWN, &mut tile)
            } else if x == 0 && z == size - 1 {
                fence_model = format!("fence_corner_{fence_index}");
                transform_with_direction(UP, &mut tile)
            } else if x == size - 1 && z == 0 {
                fence_model = format!("fence_corner_{fence_index}");
                transform_with_direction(LEFT, &mut tile)
            } else if x == size - 1 && z == size - 1 {
                fence_model = format!("fence_corner_{fence_index}");
                transform_with_direction(RIGHT, &mut tile)
            } else if x == 0 || x == size - 1 || z == 0 || z == size - 1 {
                let fence_index = rng.random_range(1..5);
                fence_model = format!("fence_{fence_index}");

                if x == 0 || x == size - 1 {
                    tile = tile.with_rotation(Quat::from_rotation_y(1.5707963268));
                }
            }

            if !fence_model.is_empty() {
                let path = format!("models/infra/{}.glb", fence_model);

                let fence_entity = spawn_object(Fence, &tile, &path, &mut commands, &asset_server);

                world
                    .object
                    .entry(tile_pos)
                    .or_insert_with(Vec::new)
                    .push(fence_entity);
            }
        }
    }
}
