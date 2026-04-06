use crate::{
    extra::noise::perlin_2d,
    world::{
        components::{Noise, StaticWorld, TILE_SIZE, TileType, TileWorld},
        utils::{
            add_component_to_entity, apply_rotation, apply_scale, apply_transformations,
            model_path, range_from_surfaces, spawn_object,
        },
    },
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

pub fn spawn_patches(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    for block in &static_world.blocks {
        if let TileType::Patches(config) = &block.tiletype {
            let init = TILE_SIZE / (config.value_2.1 * 2) as f32;
            let step = TILE_SIZE / config.value_2.1 as f32;

            let range = range_from_surfaces(&block.surface);

            'tiles: for tile in range {
                let transform = tile.to_world_transform();
                let pos = transform.translation - 2.0 + init;

                for z in 0..config.value_2.1 {
                    for x in 0..config.value_2.1 {
                        let x = pos.x + x as f32 * step;
                        let z = pos.z + z as f32 * step;

                        let point = noise_point(config, x, z);

                        if point < config.value_2.0 {
                            continue;
                        }

                        let index = rng.random_range(0..config.value_1.len());
                        let object = &config.value_1[index];

                        let mut transform = tile.to_world_transform();
                        apply_rotation(&mut rng, &mut transform, &object.placement);
                        apply_scale(&mut rng, &mut transform, &object.placement);

                        transform.translation.x = x + rng.random_range(-1.0..1.0) * init;
                        transform.translation.z = z + rng.random_range(-1.0..1.0) * init;

                        let path = model_path(&mut rng, &object);
                        let id = spawn_object(&transform, &path, &mut commands, &assets);

                        add_component_to_entity(&mut commands, &object.comp, id);
                        world.models.entry(tile).or_default().push(id);
                    }
                }
            }
        }
    }
}

fn spawn_debug(
    pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
        Transform::from_xyz(pos.x, pos.y, pos.z),
    ));
}

fn noise_point<T, K>(noise: &Noise<T, K>, x: f32, z: f32) -> f32 {
    let mut value = 0.0;
    let mut max = 0.0;

    for octave in &noise.octaves {
        let noise = perlin_2d(x * octave.frequency, z * octave.frequency);
        value += noise * octave.amplitude;
        max += octave.amplitude;
    }

    value /= max;

    ((value + 1.0) / 2.0).clamp(0.0, 1.0)
}
