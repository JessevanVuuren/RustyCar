pub mod components;
pub mod spawn;

use bevy::{color::palettes::css::RED, ecs::query::QueryIter, light::NotShadowCaster, prelude::*, transform};
use rand::{RngExt, SeedableRng, rngs::SmallRng, seq::{IndexedRandom, IteratorRandom}};

use crate::world::{
    components::{Fence, Flower, Ground, TileMap, TilePos},
    spawn::spawn_object,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileMap>();
        app.add_systems(
            Startup,
            ((generate_ground, generate_fencing), generate_flowers).chain(),
        );
    }
}

fn generate_flowers(
    mut commands: Commands,
    mut world: ResMut<TileMap>,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Ground>>,
) {
    let amount = 10;
    let mut rng = SmallRng::seed_from_u64(1604);

    let grounds: Vec<&Transform> = query.iter().collect();

    for _ in 0..amount {
        if let Some(transform) = grounds.choose(&mut rng) {

            let tile = TilePos::transform_to_tile(transform);
            let pos = tile.to_random_world_transform(&mut rng);

            let flower_entity = spawn_object(
                Flower,
                &pos,
                "models/nature/flower_1.glb",
                &mut commands,
                &asset_server,
            );

            world
                .object
                .entry(tile)
                .or_insert_with(Vec::new)
                .push(flower_entity);
        }
    }
}

fn generate_ground(
    mut commands: Commands,
    mut world: ResMut<TileMap>,
    asset_server: Res<AssetServer>,
) {
    let size = 10;
    let half = size / 2;

    let mut rng = SmallRng::seed_from_u64(1604);

    for x in 0..size {
        for z in 0..size {
            let tile_pos = TilePos {
                x: x - half,
                z: z - half,
            };

            let mut transform = tile_pos.to_world_transform();

            let ground_index = rng.random_range(1..8);
            let path = format!("models/ground/grass_{}.glb", ground_index);

            let angle = rng.random_range(0..5);
            transform = transform.with_rotation(Quat::from_rotation_y(1.5707963268 * angle as f32));

            let ground = commands
                .spawn((
                    Ground,
                    transform,
                    SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
                ))
                .id();

            world.ground.insert(tile_pos, ground);
        }
    }
}

fn generate_fencing(
    mut commands: Commands,
    mut world: ResMut<TileMap>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    let size = 10;
    let half = size / 2;

    for x in 0..size {
        for z in 0..size {
            let tile_pos = TilePos {
                x: x - half,
                z: z - half,
            };
            let mut transform = tile_pos.to_world_transform();

            let mut fence_model = "".into();

            if x == 0 && z == 0 {
                let fence_index = rng.random_range(1..3);
                fence_model = format!("fence_corner_{fence_index}");
                transform = transform.with_rotation(Quat::from_rotation_y(-1.5707963268));
            } else if x == 0 && z == size - 1 {
                let fence_index = rng.random_range(1..3);
                fence_model = format!("fence_corner_{fence_index}");
                transform = transform.with_rotation(Quat::from_rotation_y(0.0));
            } else if x == size - 1 && z == 0 {
                let fence_index = rng.random_range(1..3);
                fence_model = format!("fence_corner_{fence_index}");
                transform = transform.with_rotation(Quat::from_rotation_y(3.1415926536));
            } else if x == size - 1 && z == size - 1 {
                let fence_index = rng.random_range(1..3);
                fence_model = format!("fence_corner_{fence_index}");
                transform = transform.with_rotation(Quat::from_rotation_y(1.5707963268));
            } else if x == 0 || x == size - 1 || z == 0 || z == size - 1 {
                let fence_index = rng.random_range(1..5);
                fence_model = format!("fence_{fence_index}");

                if x == 0 || x == size - 1 {
                    transform = transform.with_rotation(Quat::from_rotation_y(1.5707963268));
                }
            }

            if !fence_model.is_empty() {
                let path = format!("models/infra/{}.glb", fence_model);

                let fence = commands
                    .spawn((
                        Fence,
                        transform,
                        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
                    ))
                    .id();

                world
                    .object
                    .entry(tile_pos)
                    .or_insert_with(Vec::new)
                    .push(fence);
            }
        }
    }
}
