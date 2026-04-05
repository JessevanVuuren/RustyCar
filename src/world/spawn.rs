use crate::world::{
    components::{
        BASE_ASSET, Comp, Dirt, Fence, Flower, Ground, Land, LandConfig, Log, Model, Mushroom,
        Object, Offset, Placement, Range, Rock, Rotation, StaticWorld, TilePos, TileType,
        TileWorld, Tree, Value,
    },
    ground::ground::ground_plane,
    utils::range_from_surface,
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{f32::consts::FRAC_PI_2, iter, slice::from_ref};

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
        let models = match &block.models {
            TileType::Ground(ground) => from_ref(ground),
            TileType::Models(models) => models,
        };

        for object in models {
            let range = range_from_surface(&block.surface);

            match object.placement.amount {
                Value::Random(low, high) => {
                    todo!("Random not implemented")
                }
                Value::True => {
                    for tile in range {
                        let path = model_path(&mut rng, &object);

                        let mut transform = tile.to_world_transform();
                        let placement = &object.placement;
                        apply_transformations(&mut rng, &mut transform, placement);

                        let comp = object.comp.clone();

                        let id = match comp {
                            Comp::Land(config) => spawn_grass(
                                &mut rng,
                                config,
                                &transform,
                                &mut commands,
                                &mut meshes,
                                &mut materials,
                            ),
                            _ => spawn_object(comp, &transform, &path, &mut commands, &assets),
                        };

                        add_to_world_map(tile, &block.models, &mut world, id, layer_id)
                    }
                }
                Value::Amount(amount) => {
                    let tiles: Vec<TilePos> = range.collect();
                    for _ in 0..amount {
                        let path = model_path(&mut rng, &object);
                        let tile = tiles[rng.random_range(0..tiles.len())];

                        let mut transform = tile.to_world_transform();

                        let placement = &object.placement;
                        apply_transformations(&mut rng, &mut transform, placement);

                        let comp = object.comp.clone();
                        let id = spawn_object(comp, &transform, &path, &mut commands, &assets);
                        add_to_world_map(tile, &block.models, &mut world, id, layer_id)
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
        TileType::Models(_) => world.object.entry(key).or_default().push(id),
        TileType::Ground(_) => {
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
    config: LandConfig,
    transform: &Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let grass = ground_plane(rng, transform.translation, 4, 4.0, config);

    commands
        .spawn((
            Land,
            Mesh3d(meshes.add(grass)),
            MeshMaterial3d(materials.add(Color::WHITE)),
            transform.clone(),
        ))
        .id()
}

fn spawn_object(
    component: Comp,
    transform: &Transform,
    path: &str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let id = commands
        .spawn((
            Object,
            transform.clone(),
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.to_string()))),
        ))
        .id();

    match component {
        Comp::Mushroom => commands.entity(id).insert(Mushroom),
        Comp::Flower => commands.entity(id).insert(Flower),
        Comp::Fence => commands.entity(id).insert(Fence),
        Comp::Tree => commands.entity(id).insert(Tree),
        Comp::Rock => commands.entity(id).insert(Rock),
        Comp::Dirt => commands.entity(id).insert(Dirt),
        Comp::Log => commands.entity(id).insert(Log),
        Comp::None => panic!("None is invalid Component type"),
        Comp::Land(grass_config) => panic!("Non configurable grass Tile"),
    };

    id
}

fn model_path(rng: &mut SmallRng, model: &Model) -> String {
    let path = format!("{BASE_ASSET}{}", model.path);

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
