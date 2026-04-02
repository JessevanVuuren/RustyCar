use crate::{
    extra::math::{ease_in_quint, lerp, s_curve},
    world::{
        components::{Comp, Grass, Range, StaticWorld, TilePos, TileWorld},
        ground::mesh_utils::{set_mesh_colors, tile_mesh_colors, tile_mesh_positions},
        utils::range_from_surface,
    },
};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use rand_distr::{Distribution, Normal};

use std::{
    collections::HashMap,
    f32::consts::{FRAC_2_PI, FRAC_PI_2, PI},
    thread,
    time::Instant,
};

use bevy::{math::ops::sqrt, mesh::VertexAttributeValues, prelude::*};
use std::{collections::HashSet, iter};

pub fn color_fade(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Mesh3d, With<Grass>>,
) {
    let now = Instant::now();
    let mut rng = SmallRng::seed_from_u64(1604);
    let mut normal = Normal::new(0.0, 1.0).unwrap();

    let sub_quads = 2i32.pow(4.0 as u32);
    let points = sub_quads;

    let mut avg_colors = HashMap::new();

    for i in 0..static_world.blocks.iter().len() {
        let block = &static_world.blocks[i];
        let object = &block.objects[0];

        match &object.comp {
            Comp::Grass(config) => {
                for surface in block.surface.iter() {
                    let range = range_from_surface(surface);

                    'tiles: for tile in range {
                        let ground = world.ground.get(&tile).map_or(0, |f| f.id);

                        if let Some((mut color, handler)) =
                            tile_mesh_colors(&world, tile, &query, &meshes)
                        {
                            let info = tile_color_info(&color);
                            avg_colors.insert(tile, info);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    for i in 0..static_world.blocks.iter().len() {
        let block = &static_world.blocks[i];
        let object = &block.objects[0];

        match &object.comp {
            Comp::Grass(config) => {
                for surface in block.surface.iter() {
                    let range = range_from_surface(surface);

                    'tiles: for tile in range {
                        let mut tile_tl = TilePos::new(tile.x + 0, tile.z + 0); // top      left
                        let mut tile_tr = TilePos::new(tile.x + 1, tile.z + 0); // top      right
                        let mut tile_bl = TilePos::new(tile.x + 0, tile.z + 1); // bottom   left
                        let mut tile_br = TilePos::new(tile.x + 1, tile.z + 1); // bottom   right

                        let ground_tl = world.ground.get(&tile_tl).map_or(0, |f| f.id); // top      left
                        let ground_tr = world.ground.get(&tile_tr).map_or(0, |f| f.id); // top      right
                        let ground_bl = world.ground.get(&tile_bl).map_or(0, |f| f.id); // bottom   left
                        let ground_br = world.ground.get(&tile_br).map_or(0, |f| f.id); // bottom   right

                        if let (
                            Some((mut color_tl, handler_tl)),
                            Some((mut color_tr, handler_tr)),
                            Some((mut color_bl, handler_bl)),
                            Some((mut color_br, handler_br)),
                        ) = (
                            tile_mesh_colors(&world, tile_tl, &query, &meshes),
                            tile_mesh_colors(&world, tile_tr, &query, &meshes),
                            tile_mesh_colors(&world, tile_bl, &query, &meshes),
                            tile_mesh_colors(&world, tile_br, &query, &meshes),
                        ) {
                            if ground_tl == ground_tr
                                && ground_bl == ground_br
                                && ground_bl == ground_tl
                            {
                                continue 'tiles;
                            }

                            if ground_tl == ground_tr && ground_bl == ground_br {
                                let scaler: Vec<f32> = ramp_map(points, 2.0).collect();
                                let scaler = &rotate(&scaler, points, 1);

                                mix_colors(
                                    &mut rng,
                                    &avg_colors[&tile_tl],
                                    &avg_colors[&tile_tr],
                                    &avg_colors[&tile_bl],
                                    &avg_colors[&tile_br],
                                    &mut color_tl,
                                    &mut color_tr,
                                    &mut color_bl,
                                    &mut color_br,
                                    &scaler,
                                );
                            } else if ground_tl == ground_bl && ground_tr == ground_br {
                                let scaler: Vec<f32> = ramp_map(points, 2.0).collect();
                                mix_colors(
                                    &mut rng,
                                    &avg_colors[&tile_tl],
                                    &avg_colors[&tile_bl],
                                    &avg_colors[&tile_tr],
                                    &avg_colors[&tile_br],
                                    &mut color_tl,
                                    &mut color_tr,
                                    &mut color_bl,
                                    &mut color_br,
                                    &scaler,
                                );
                            }

                            set_mesh_colors(&color_tl, &handler_tl, &mut meshes);
                            set_mesh_colors(&color_tr, &handler_tr, &mut meshes);
                            set_mesh_colors(&color_bl, &handler_bl, &mut meshes);
                            set_mesh_colors(&color_br, &handler_br, &mut meshes);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

const PRECISION: i32 = 1000;

fn color_to_key(color: [f32; 4]) -> [i32; 4] {
    [
        (color[0] * PRECISION as f32) as i32,
        (color[1] * PRECISION as f32) as i32,
        (color[2] * PRECISION as f32) as i32,
        (color[3] * PRECISION as f32) as i32,
    ]
}

fn key_to_color(key: [i32; 4]) -> [f32; 4] {
    [
        key[0] as f32 / PRECISION as f32,
        key[1] as f32 / PRECISION as f32,
        key[2] as f32 / PRECISION as f32,
        key[3] as f32 / PRECISION as f32,
    ]
}

fn color_to_attrib(c: Color) -> [f32; 4] {
    let c = c.to_linear();
    [c.red, c.green, c.blue, 1.0]
}

fn random_xz(rng: &mut SmallRng, size: i32) -> (i32, i32) {
    // let theta = rng.random::<f32>() * FRAC_PI_2;
    // let length = rng.random::<f32>().powf(0.5 * SPREAD) * size as f32;
    // (
    //     (theta.cos() * length).round() as i32,
    //     (theta.sin() * length).round() as i32,
    // )

    (
        (rng.random::<f32>() * size as f32) as i32,
        (rng.random::<f32>() * size as f32) as i32,
    )
}

fn add_colors_attrib(color1: &mut [f32; 4], color2: [f32; 4]) {
    color1[0] += color2[0];
    color1[1] += color2[1];
    color1[2] += color2[2];
    color1[3] += color2[3];
}

fn avg_color_attrib(color: &mut [f32; 4], amount: i32) {
    color[0] /= amount as f32;
    color[1] /= amount as f32;
    color[2] /= amount as f32;
    color[3] /= amount as f32;
}

fn value_color_attrib(value: f32) -> [f32; 4] {
    [value, value, value, 1.0]
}

fn lerp_color_attrib(t: f32, color1: [f32; 4], color2: [f32; 4]) -> [f32; 4] {
    [
        lerp(t, color1[0], color2[0]),
        lerp(t, color1[1], color2[1]),
        lerp(t, color1[2], color2[2]),
        lerp(t, color1[3], color2[3]),
    ]
}

const SPREAD: f32 = 1.0;
const SAMPLES: i32 = 50;

fn tile_color_info(tile: &[[f32; 4]]) -> ([f32; 4], Vec<[i32; 4]>) {
    let sub_quads = 2i32.pow(4.0 as u32);
    let points = sub_quads - 1;

    let mut color = tile[0];
    let mut colors = HashSet::new();

    for i in 0..sub_quads * 6 {
        let quad_color = tile[i as usize];
        add_colors_attrib(&mut color, quad_color);
        colors.insert(color_to_key(quad_color));
    }

    avg_color_attrib(&mut color, sub_quads * 6);

    (color, colors.iter().copied().collect())
}

fn mix_colors(
    rng: &mut SmallRng,
    info1: &([f32; 4], Vec<[i32; 4]>),
    info2: &([f32; 4], Vec<[i32; 4]>),
    info3: &([f32; 4], Vec<[i32; 4]>),
    info4: &([f32; 4], Vec<[i32; 4]>),
    tile1: &mut [[f32; 4]],
    tile2: &mut [[f32; 4]],
    tile3: &mut [[f32; 4]],
    tile4: &mut [[f32; 4]],
    scaler: &Vec<f32>,
) {
    let sub_quads = 2i32.pow(4.0 as u32);
    let points = sub_quads - 1;

    for _ in 0..SAMPLES {
        let (x, z) = random_xz(rng, points);
        let i = ((points - z) * sub_quads + (points - x)) as usize;

        let color = info3.1[rng.random_range(0..info3.1.len()) as usize];
        let color_lerp = lerp_color_attrib(scaler[i], info1.0, key_to_color(color));

        tile1[i * 6 + 0] = color_lerp;
        tile1[i * 6 + 1] = color_lerp;
        tile1[i * 6 + 2] = color_lerp;
        tile1[i * 6 + 3] = color_lerp;
        tile1[i * 6 + 4] = color_lerp;
        tile1[i * 6 + 5] = color_lerp;

        let (x, z) = random_xz(rng, points);
        let i = ((points - z) * sub_quads + x) as usize;

        let color = info4.1[rng.random_range(0..info4.1.len()) as usize];
        let color_lerp = lerp_color_attrib(scaler[i], info2.0, key_to_color(color));

        tile2[i * 6 + 0] = color_lerp;
        tile2[i * 6 + 1] = color_lerp;
        tile2[i * 6 + 2] = color_lerp;
        tile2[i * 6 + 3] = color_lerp;
        tile2[i * 6 + 4] = color_lerp;
        tile2[i * 6 + 5] = color_lerp;

        let (x, z) = random_xz(rng, points);
        let i = (z * sub_quads + (points - x)) as usize;

        let color = info1.1[rng.random_range(0..info1.1.len()) as usize];
        let color_lerp = lerp_color_attrib(1.0 - scaler[i], info3.0, key_to_color(color));

        tile3[i * 6 + 0] = color_lerp;
        tile3[i * 6 + 1] = color_lerp;
        tile3[i * 6 + 2] = color_lerp;
        tile3[i * 6 + 3] = color_lerp;
        tile3[i * 6 + 4] = color_lerp;
        tile3[i * 6 + 5] = color_lerp;

        let (x, z) = random_xz(rng, points);
        let i = (z * sub_quads + x) as usize;

        let color = info2.1[rng.random_range(0..info2.1.len()) as usize];
        let color_lerp = lerp_color_attrib(1.0 - scaler[i], info4.0, key_to_color(color));

        tile4[i * 6 + 0] = color_lerp;
        tile4[i * 6 + 1] = color_lerp;
        tile4[i * 6 + 2] = color_lerp;
        tile4[i * 6 + 3] = color_lerp;
        tile4[i * 6 + 4] = color_lerp;
        tile4[i * 6 + 5] = color_lerp;
    }
}

fn ramp_map(size: i32, intensity: f32) -> impl Iterator<Item = f32> {
    (0..size * size).map(move |i| (i % size) as f32 / (size - 1) as f32)
}

fn rotate<T: Copy>(array: &[T], size: i32, step: usize) -> Vec<T> {
    let mut rotated = Vec::with_capacity((size * size) as usize);

    for y in 0..size {
        for x in 0..size {
            let i = match step {
                0 => y * size + x,
                1 => (size - 1 - x) * size + y,
                2 => (size - y) * size - 1 - x,
                3 => (size - 1 - y) + size * x,
                _ => panic!("Unreachable step: {}", step),
            };
            rotated.push(array[i as usize].clone());
        }
    }

    rotated
}

fn place_helper_points_horizontal(
    height_map: &Vec<f32>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    offset: &TilePos,
) {
    let size = (height_map.len() as f32).sqrt();

    let offset_x = (offset.x * 4) as f32 - 2.0;
    let offset_z = (offset.z * 4) as f32 - 2.0;
    let offset_y = 1.0;
    let step = 4.0 / (size - 1.0) as f32;

    for (i, p) in height_map.iter().enumerate() {
        let x = (i % size as usize) as f32;
        let z = (i / size as usize) as f32;

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.05))),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
            Transform::from_xyz(offset_x + x * step, offset_y + p, offset_z + z * step),
        ));
    }
}
