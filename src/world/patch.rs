use crate::world::components::{
    Comp, Grass, GrassConfig, Model, Offset, Placement, Range, Rotation, StaticWorld, TILE_SIZE,
    TilePos, TileType, TileWorld, Value,
};
use bevy::{mesh::VertexAttributeValues, prelude::*};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{collections::HashMap, f32::consts::FRAC_PI_2, iter, str::LinesAny};

// let CONFIGURATION: [[usize; 4]; 2] = [[1, 1, 0, 0], [1,1,1,0]];

// 3 type of ramps
// 4 ramps
// 4 rotations
// 4 tiles

enum Stitch {
    Ramp,
    In,
    Out,
}

struct PatchWorks {
    template: [usize; 4],
    stitch: [Stitch; 4],
    rotation: [usize; 4],
}

const MID_POINT: f32 = 0.25;

const CONFIGURATION: [PatchWorks; 6] = [
    PatchWorks {
        stitch: [Stitch::Ramp, Stitch::Ramp, Stitch::Ramp, Stitch::Ramp],
        rotation: [1, 1, 3, 3],
        template: [1, 1, 0, 0],
    },
    PatchWorks {
        stitch: [Stitch::Ramp, Stitch::Ramp, Stitch::Ramp, Stitch::Ramp],
        rotation: [0, 2, 0, 2],
        template: [1, 0, 1, 0],
    },
    PatchWorks {
        stitch: [Stitch::Out, Stitch::Ramp, Stitch::Ramp, Stitch::In],
        rotation: [0, 1, 0, 2],
        template: [1, 1, 1, 0],
    },
    PatchWorks {
        stitch: [Stitch::Ramp, Stitch::Out, Stitch::In, Stitch::Ramp],
        rotation: [1, 1, 3, 2],
        template: [1, 1, 0, 1],
    },
    PatchWorks {
        stitch: [Stitch::Ramp, Stitch::In, Stitch::Out, Stitch::Ramp],
        rotation: [0, 1, 3, 3],
        template: [1, 0, 1, 1],
    },
    PatchWorks {
        stitch: [Stitch::In, Stitch::Ramp, Stitch::Ramp, Stitch::Out],
        rotation: [0, 2, 3, 2],
        template: [0, 1, 1, 1],
    },
];

pub fn patch_ground(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Mesh3d, With<Grass>>,
) {
    for block in static_world.blocks.iter() {
        let object = &block.objects[0];

        match &object.comp {
            Comp::Grass(config) => {
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

                    for tile in range {
                        let mut tile_tl = TilePos::new(tile.x + 0, tile.z + 0); // top      left
                        let mut tile_tr = TilePos::new(tile.x + 1, tile.z + 0); // top      right
                        let mut tile_bl = TilePos::new(tile.x + 0, tile.z + 1); // bottom   left
                        let mut tile_br = TilePos::new(tile.x + 1, tile.z + 1); // bottom   right

                        let ground_tl = world.ground.get(&tile_tl).map_or(0, |f| f.id); // top      left
                        let ground_tr = world.ground.get(&tile_tr).map_or(0, |f| f.id); // top      right
                        let ground_bl = world.ground.get(&tile_bl).map_or(0, |f| f.id); // bottom   left
                        let ground_br = world.ground.get(&tile_br).map_or(0, |f| f.id); // bottom   right

                        if let (
                            Some(mut pos_tl),
                            Some(mut pos_tr),
                            Some(mut pos_bl),
                            Some(mut pos_br),
                        ) = (
                            tile_mesh_positions(&world, tile_tl, &query, &meshes),
                            tile_mesh_positions(&world, tile_tr, &query, &meshes),
                            tile_mesh_positions(&world, tile_bl, &query, &meshes),
                            tile_mesh_positions(&world, tile_br, &query, &meshes),
                        ) {
                            if ground_tl == ground_tr
                                && ground_bl == ground_br
                                && ground_bl == ground_tl
                            {
                                continue;
                            }

                            let sub_quads = 2i32.pow(4.0 as u32);
                            let half = sub_quads / 2;
                            let points = half + 1;

                            for conf in CONFIGURATION {
                                let inverted = inverse_template(&conf.template);

                                let observed = [ground_tl, ground_tr, ground_bl, ground_br];

                                let res_1 = multiply_core(&conf.template, &observed);
                                let res_2 = multiply_core(&inverted, &observed);

                                let res_1 = core_continuity(&res_1);
                                let res_2 = core_continuity(&res_2);

                                if res_1 && res_2 {
                                    let mut height_map: Vec<Vec<f32>> = Vec::new();

                                    for i in 0..4 {
                                        let map: Vec<f32> = match conf.stitch[i] {
                                            Stitch::Ramp => ramp_map(points).collect(),
                                            Stitch::In => reverse_map(points).collect(),
                                            Stitch::Out => curve_map(points).collect(),
                                        };

                                        height_map.push(rotate(map, points, conf.rotation[i]));
                                    }

                                    stitch_tiles(
                                        &mut pos_tl.0,
                                        &mut pos_tr.0,
                                        &mut pos_bl.0,
                                        &mut pos_br.0,
                                        height_map,
                                    );

                                    set_mesh_position(&pos_tl.0, &pos_tl.1, &mut meshes);
                                    set_mesh_position(&pos_tr.0, &pos_tr.1, &mut meshes);
                                    set_mesh_position(&pos_bl.0, &pos_bl.1, &mut meshes);
                                    set_mesh_position(&pos_br.0, &pos_br.1, &mut meshes);
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn stitch_tiles(
    tile1: &mut [[f32; 3]],
    tile2: &mut [[f32; 3]],
    tile3: &mut [[f32; 3]],
    tile4: &mut [[f32; 3]],
    height_map: Vec<Vec<f32>>,
) {
    let sub_quads = 2i32.pow(4.0 as u32);
    let half = sub_quads / 2;
    let points = half + 1;

    for z in 0..half {
        for x in 0..half {
            let index = (z * half + x) as usize;

            let base_1 = (z * points + x) as usize;
            let base_2 = ((z + 1) * points + x) as usize;

            let top_left = height_map[0][base_1 + 0];
            let bot_left = height_map[0][base_2 + 0];

            let top_right = height_map[0][base_1 + 1];
            let bot_right = height_map[0][base_2 + 1];

            let i = (((z + half) * sub_quads + (x + half)) * 6) as usize;
            quad_to_triangle(tile1, &height_map[0], base_1, base_2, i);

            let i = (((z + half) * sub_quads + x) * 6) as usize;
            quad_to_triangle(tile2, &height_map[1], base_1, base_2, i);

            let i = ((z * sub_quads + (x + half)) * 6) as usize;
            quad_to_triangle(tile3, &height_map[2], base_1, base_2, i);

            let i = ((z * sub_quads + x) * 6) as usize;
            quad_to_triangle(tile4, &height_map[3], base_1, base_2, i);
        }
    }
}

fn quad_to_triangle(
    tile1: &mut [[f32; 3]],
    height_map: &Vec<f32>,
    base_1: usize,
    base_2: usize,
    index: usize,
) {
    let top_left = ease_in_quint(height_map[base_1 + 0]);
    let bot_left = ease_in_quint(height_map[base_2 + 0]);

    let top_right = ease_in_quint(height_map[base_1 + 1]);
    let bot_right = ease_in_quint(height_map[base_2 + 1]);

    tile1[index + 0][1] = lerp(top_left, tile1[index + 0][1], MID_POINT);
    tile1[index + 1][1] = lerp(bot_left, tile1[index + 1][1], MID_POINT);
    tile1[index + 2][1] = lerp(top_right, tile1[index + 2][1], MID_POINT);
    tile1[index + 3][1] = lerp(top_right, tile1[index + 3][1], MID_POINT);
    tile1[index + 4][1] = lerp(bot_left, tile1[index + 4][1], MID_POINT);
    tile1[index + 5][1] = lerp(bot_right, tile1[index + 5][1], MID_POINT);
}

fn multiply_core(template: &[usize; 4], observed: &[usize; 4]) -> [usize; 4] {
    [
        template[0] * observed[0],
        template[1] * observed[1],
        template[2] * observed[2],
        template[3] * observed[3],
    ]
}

fn inverse_template(template: &[usize; 4]) -> [usize; 4] {
    template.map(|i| if i == 0 { 1 } else { 0 })
}

fn core_continuity(result: &[usize; 4]) -> bool {
    let mut base = 0 as usize;
    for i in 0..4 {
        if base == 0 && result[i] != 0 {
            base = result[i];
        }

        if result[i] != 0 && result[i] != base {
            return false;
        }
    }

    true
}

fn rotate<T: Copy>(array: Vec<T>, size: i32, step: usize) -> Vec<T> {
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
            rotated.push(array[i as usize]);
        }
    }

    rotated
}

#[inline]
fn relu(v: f32) -> f32 {
    if v > 0.0 { v } else { 0.0 }
}

fn ramp_map(size: i32) -> impl Iterator<Item = f32> {
    (0..size * size).map(move |i| {
        let x = (i % size) as f32;
        let n = (size - 1) as f32;

        x / n
    })
}

fn curve_map(size: i32) -> impl Iterator<Item = f32> {
    (0..size * size).map(move |i| {
        let x = (i % size) as f32;
        let y = (i / size) as f32;
        let n = (size - 1) as f32;

        x * y / (n * n)
    })
}

fn reverse_map(size: i32) -> impl Iterator<Item = f32> {
    (0..size * size).map(move |i| {
        let x = (i % size) as f32 / (size - 1) as f32;
        let y = (i / size) as f32 / (size - 1) as f32;

        x + y - x * y
    })
}

#[inline]
fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

#[inline]
fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - ((-2.0 * x + 2.0) as f32).powf(3.0) / 2.0
    }
}

#[inline]
fn ease_in_out_sine(x: f32) -> f32 {
    -(f32::cos(std::f32::consts::PI * x) - 1.0) / 2.0
}

#[inline]
pub fn ease_in_out_circ(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

#[inline]
fn ease_in_quint(x: f32) -> f32 {
    // x * x
    let x2 = x * x;
    let x4 = x2 * x2;
    x4 * x4
    // let x8 = x4 * x4;
    // let x16 = x8 * x8;
    // x16 * x16
}

fn tile_mesh_positions(
    world: &TileWorld,
    tile: TilePos,
    query: &Query<&Mesh3d, With<Grass>>,
    meshes: &Assets<Mesh>,
) -> Option<(Vec<[f32; 3]>, Handle<Mesh>)> {
    if let Some(handle) = mesh_on_tile(&world, tile, &query) {
        if let Some(mut pos) = get_mesh_positions(&meshes, &handle) {
            return Some((pos, handle));
        }
    }

    None
}

fn set_mesh_position(positions: &Vec<[f32; 3]>, handle: &Handle<Mesh>, meshes: &mut Assets<Mesh>) {
    if let Some(mesh) = meshes.get_mut(handle) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.compute_flat_normals();
    }
}

fn get_mesh_positions(meshes: &Assets<Mesh>, handle: &Handle<Mesh>) -> Option<Vec<[f32; 3]>> {
    let mesh = meshes.get(handle)?;
    match mesh.attribute(Mesh::ATTRIBUTE_POSITION)? {
        VertexAttributeValues::Float32x3(pos) => Some(pos.clone()),
        _ => None,
    }
}

fn mesh_on_tile(
    world: &TileWorld,
    tile: TilePos,
    query: &Query<&Mesh3d, With<Grass>>,
) -> Option<Handle<Mesh>> {
    if let Some(t) = world.ground.get(&tile) {
        if let Ok(mesh3d) = query.get(t.entity.entity()) {
            return Some(mesh3d.0.clone());
        }
    }

    None
}
