use crate::world::{
    components::{
        Comp, Grass, GrassConfig, Ground, Model, Offset, Placement, Range, Rotation, StaticWorld,
        TilePos, TileType, TileWorld, Value,
    },
    grass::ground_plane,
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{f32::consts::FRAC_PI_2, iter};

const BASE: &str = "models/";

pub fn init_static_world(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    for (layer_id, block) in static_world.blocks.iter().enumerate() {
        for object in block.objects.iter() {
            for surface in block.surface.iter() {
                let positive = match surface.positive {
                    Range::None => panic!("Surface range cant be None"),
                    Range::Range(start, stop) => start.row_major(stop),
                    Range::One(place) => place.row_major(place),
                };

                let negative: Box<dyn Iterator<Item = TilePos>> = match surface.negative {
                    Range::None => Box::new(iter::empty()),
                    Range::Range(start, stop) => Box::new(start.row_major(stop)),
                    Range::One(place) => Box::new(place.row_major(place)),
                };

                let range = TilePos::subtract_range(positive, negative);

                match object.placement.amount {
                    Value::Random(low, high) => {}
                    Value::True => {
                        for tile in range {
                            let path = model_path(&mut rng, object);

                            let mut transform = tile.to_world_transform();
                            let placement = &object.placement;
                            apply_transformations(&mut rng, &mut transform, placement);

                            let comp = object.comp.clone();

                            let id = match comp {
                                Comp::Grass(config) => spawn_grass(
                                    &mut rng,
                                    config,
                                    &transform,
                                    &mut commands,
                                    &mut meshes,
                                    &mut materials,
                                ),
                                _ => spawn_object(comp, &transform, &path, &mut commands, &assets),
                            };

                            add_to_world_map(tile, &object.tile_type, &mut world, id, layer_id)
                        }
                    }
                    Value::Amount(amount) => {
                        let tiles: Vec<TilePos> = range.collect();
                        for _ in 0..amount {
                            let path = model_path(&mut rng, object);
                            let tile = tiles[rng.random_range(0..tiles.len())];

                            let mut transform = tile.to_world_transform();

                            let placement = &object.placement;
                            apply_transformations(&mut rng, &mut transform, placement);

                            let comp = object.comp.clone();
                            let id = spawn_object(comp, &transform, &path, &mut commands, &assets);
                            add_to_world_map(tile, &object.tile_type, &mut world, id, layer_id)
                        }
                    }
                }
            }
        }
    }
}

fn add_to_world_map(
    key: TilePos,
    tile_type: &TileType,
    world: &mut TileWorld,
    id: Entity,
    layer_id: usize,
) {
    match tile_type {
        TileType::Object => world.object.entry(key).or_default().push(id),
        TileType::Ground => {
            world.ground.insert(
                key,
                Ground {
                    entity: id,
                    id: layer_id + 1,
                },
            );
        }
    };
}

fn spawn_grass(
    rng: &mut SmallRng,
    config: GrassConfig,
    transform: &Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let grass = ground_plane(rng, transform.translation, 4, 4.0, config);

    commands
        .spawn((
            Grass,
            Mesh3d(meshes.add(grass)),
            MeshMaterial3d(materials.add(Color::WHITE)),
            transform.clone(),
        ))
        .id()
}

fn spawn_object<T: Component>(
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

fn model_path(rng: &mut SmallRng, model: &Model) -> String {
    let path = format!("{BASE}{}", model.path);

    match model.range {
        Range::None => format!("{path}.glb"),
        Range::One(i) => format!("{path}_{i}.glb"),
        Range::Range(a, b) => format!("{path}_{}.glb", rng.random_range(a..=b)),
    }
}

fn apply_transformations(rng: &mut SmallRng, transform: &mut Transform, placement: &Placement) {
    match placement.rotation {
        Rotation::Amount(angle, axis) => transform.rotate_axis(axis, angle),
        Rotation::Random(a, b) => {
            let angle = rng.random_range(a..b);
            transform.rotation *= Quat::from_rotation_y(angle);
        }
        Rotation::RandomDirection => {
            let angle = rng.random_range(0..=3);

            transform.rotation *= Quat::from_rotation_y(angle as f32 * FRAC_PI_2);
        }
        Rotation::True => {}
    }

    match placement.offset {
        Offset::Fixed(vec) => transform.translation += vec,
        Offset::Random => transform.translation += TilePos::random_tile_offset(rng),
        Offset::Zero => (),
    }

    match placement.scale {
        Value::Amount(amount) => transform.scale *= amount,
        Value::Random(a, b) => transform.scale *= rng.random_range(a..b),
        Value::True => (),
    }
}
