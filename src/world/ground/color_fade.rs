use crate::{
    extra::{
        math::{ease_in_quint, lerp, s_curve},
        utils::rotate,
    },
    world::{
        tile_pos::TilePos,
        components::{
            COLOR_PRECISION, Comp, Ground, Range, StaticWorld, TileType, TileWorld,
        },
        ground::mesh_utils::{set_mesh_colors, tile_mesh_colors, tile_mesh_positions},
        utils::range_from_surfaces,

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
    query: Query<&Mesh3d, With<Ground>>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);
    let mut avg_colors = HashMap::new();

    for block in &static_world.blocks {
        if let TileType::Ground(config) = &block.tiletype {
            let range = range_from_surfaces(&block.surface);
            let sub_quads = 2i32.pow(config.subdivisions as u32);

            'tiles: for tile in range {
                let ground = world.ground.get(&tile).map_or(0, |f| f.id);

                if avg_colors.contains_key(&ground) {
                    continue 'tiles;
                }

                if let Some((mut color, handler)) = tile_mesh_colors(&world, tile, &query, &meshes)
                {
                    let info = tile_color_info(&color, sub_quads);
                    avg_colors.insert(ground, info);
                }
            }
        }
    }

    for block in &static_world.blocks {
        if let TileType::Ground(config) = &block.tiletype {
            let sub_quads = 2i32.pow(config.subdivisions as u32);
            let points = sub_quads;

            let map_tl: Vec<f32> = corner_map(points, config.color_spread).collect();
            let map_tr = rotate(&map_tl, points, 1);
            let map_bl = rotate(&map_tr, points, 2);
            let map_br = rotate(&map_bl, points, 3);

            let range = range_from_surfaces(&block.surface);

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
                    Some((mut col_tl, handler_tl)),
                    Some((mut col_tr, handler_tr)),
                    Some((mut col_bl, handler_bl)),
                    Some((mut col_br, handler_br)),
                ) = (
                    tile_mesh_colors(&world, tile_tl, &query, &meshes),
                    tile_mesh_colors(&world, tile_tr, &query, &meshes),
                    tile_mesh_colors(&world, tile_bl, &query, &meshes),
                    tile_mesh_colors(&world, tile_br, &query, &meshes),
                ) {
                    if ground_tl == ground_tr && ground_bl == ground_br && ground_bl == ground_tl {
                        continue 'tiles;
                    }

                    let info_tl = &avg_colors[&ground_tl];
                    let info_tr = &avg_colors[&ground_tr];
                    let info_bl = &avg_colors[&ground_bl];
                    let info_br = &avg_colors[&ground_br];

                    let mut paint = Vec::new();
                    paint.extend_from_slice(&info_tl.1);
                    paint.extend_from_slice(&info_tr.1);
                    paint.extend_from_slice(&info_bl.1);
                    paint.extend_from_slice(&info_br.1);

                    let samples = config.color_samples;
                    mix_tile(
                        &mut rng,
                        &mut col_tl,
                        &info_tl.0,
                        &paint,
                        &map_tl,
                        samples,
                        sub_quads,
                    );
                    mix_tile(
                        &mut rng,
                        &mut col_tr,
                        &info_tr.0,
                        &paint,
                        &map_tr,
                        samples,
                        sub_quads,
                    );
                    mix_tile(
                        &mut rng,
                        &mut col_br,
                        &info_br.0,
                        &paint,
                        &map_br,
                        samples,
                        sub_quads,
                    );
                    mix_tile(
                        &mut rng,
                        &mut col_bl,
                        &info_bl.0,
                        &paint,
                        &map_bl,
                        samples,
                        sub_quads,
                    );

                    set_mesh_colors(&col_tl, &handler_tl, &mut meshes);
                    set_mesh_colors(&col_tr, &handler_tr, &mut meshes);
                    set_mesh_colors(&col_bl, &handler_bl, &mut meshes);
                    set_mesh_colors(&col_br, &handler_br, &mut meshes);
                }
            }
        }
    }
}

fn color_to_key(color: [f32; 4]) -> [i32; 4] {
    [
        (color[0] * COLOR_PRECISION as f32) as i32,
        (color[1] * COLOR_PRECISION as f32) as i32,
        (color[2] * COLOR_PRECISION as f32) as i32,
        (color[3] * COLOR_PRECISION as f32) as i32,
    ]
}

fn key_to_color(key: [i32; 4]) -> [f32; 4] {
    [
        key[0] as f32 / COLOR_PRECISION as f32,
        key[1] as f32 / COLOR_PRECISION as f32,
        key[2] as f32 / COLOR_PRECISION as f32,
        key[3] as f32 / COLOR_PRECISION as f32,
    ]
}

fn random_xz(rng: &mut SmallRng, size: i32) -> (i32, i32) {
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

fn lerp_color_attrib(t: f32, color1: [f32; 4], color2: [f32; 4]) -> [f32; 4] {
    [
        lerp(t, color1[0], color2[0]),
        lerp(t, color1[1], color2[1]),
        lerp(t, color1[2], color2[2]),
        lerp(t, color1[3], color2[3]),
    ]
}

fn tile_color_info(tile: &[[f32; 4]], sub_quads: i32) -> ([f32; 4], Vec<[i32; 4]>) {
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

fn mix_tile(
    rng: &mut SmallRng,
    tile: &mut [[f32; 4]],
    average: &[f32; 4],
    colors: &Vec<[i32; 4]>,
    scaler: &Vec<f32>,
    samples: i32,
    sub_quads: i32,
) {
    let points = sub_quads - 1;

    for _ in 0..samples {
        let (x, z) = random_xz(rng, points);
        let i = (z * sub_quads + x) as usize;

        let color_rand = colors[rng.random_range(0..colors.len()) as usize];
        let color_lerp = lerp_color_attrib(scaler[i], *average, key_to_color(color_rand));

        tile[i * 6 + 0] = color_lerp;
        tile[i * 6 + 1] = color_lerp;
        tile[i * 6 + 2] = color_lerp;
        tile[i * 6 + 3] = color_lerp;
        tile[i * 6 + 4] = color_lerp;
        tile[i * 6 + 5] = color_lerp;
    }
}

fn ramp_map(size: i32, intensity: f32) -> impl Iterator<Item = f32> {
    (0..size * size).map(move |i| (i % size) as f32 / (size - 1) as f32)
}

fn corner_map(size: i32, intensity: f32) -> impl Iterator<Item = f32> {
    let norm = size as f32 - 1.0;
    (0..size * size).map(move |i| {
        let x = ((i % size) as f32 / norm).powf(intensity);
        let y = ((i / size) as f32 / norm).powf(intensity);

        y * x
    })
}
