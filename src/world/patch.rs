use crate::world::components::{
    Comp, Grass, GrassConfig, Model, Offset, Placement, Range, Rotation, StaticWorld, TILE_SIZE,
    TilePos, TileType, TileWorld, Value,
};
use bevy::{mesh::VertexAttributeValues, prelude::*};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{f32::consts::FRAC_PI_2, iter};

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
                        let mut tile_cc = TilePos::new(tile.x + 0, tile.z + 0); // center   center
                        let mut tile_cr = TilePos::new(tile.x + 1, tile.z + 0); // right    center
                        let mut tile_bc = TilePos::new(tile.x + 0, tile.z + 1); // center   bottom
                        let mut tile_br = TilePos::new(tile.x + 1, tile.z + 1); // right    bottom

                        let ground_cc = world.ground.get(&tile_cc).map_or(-1, |f| f.id); // center center   
                        let ground_cr = world.ground.get(&tile_cr).map_or(-1, |f| f.id); // center right    
                        let ground_bc = world.ground.get(&tile_bc).map_or(-1, |f| f.id); // bottom center   
                        let ground_br = world.ground.get(&tile_br).map_or(-1, |f| f.id); // bottom right    

                        let Some(handle) = mesh_on_tile(&world, tile_cc, &query) else {
                            continue;
                        };

                        let Some(mut pos) = get_mesh_positions(&meshes, &handle) else {
                            continue;
                        };

                        // 3 type of ramps
                        // 4 ramps
                        // 4 rotations
                        // 4 tiles


                        if let (
                            Some(mut pos_cc),
                            Some(mut pos_cr),
                            Some(mut pos_bc),
                            Some(mut pos_br),
                        ) = (
                            tile_mesh_positions(&world, tile_cc, &query, &meshes),
                            tile_mesh_positions(&world, tile_cr, &query, &meshes),
                            tile_mesh_positions(&world, tile_bc, &query, &meshes),
                            tile_mesh_positions(&world, tile_br, &query, &meshes),
                        ) {
                            let sub_quads = 2i32.pow(4.0 as u32);

                            let half = sub_quads / 2;
                            let points = half + 1;

                            if (ground_cc == ground_bc
                                && ground_cc == ground_cr
                                && ground_cc != ground_br)
                            {
                                let height_ramp: Vec<f32> = ramp_map(points).collect();
                                let height_curve: Vec<f32> = curve_map(points).collect();
                                let height_reverse: Vec<f32> = reverse_map(points).collect();
                                let height_map = vec![
                                    rotate(height_curve.clone(), points, 0),
                                    rotate(height_ramp.clone(), points, 1),
                                    rotate(height_ramp.clone(), points, 0),
                                    rotate(height_reverse.clone(), points, 2),
                                ];

                                stitch_tiles(
                                    &mut pos_cc.0,
                                    &mut pos_cr.0,
                                    &mut pos_bc.0,
                                    &mut pos_br.0,
                                    height_map,
                                );
                            }

                            if (ground_cc != ground_bc
                                && ground_cr != ground_br
                                && ground_cc == ground_cr
                                && ground_bc == ground_br)
                            {
                                let height: Vec<f32> = ramp_map(points).collect();
                                let height_map = vec![
                                    rotate(height.clone(), points, 1),
                                    rotate(height.clone(), points, 1),
                                    rotate(height.clone(), points, 3),
                                    rotate(height.clone(), points, 3),
                                ];

                                stitch_tiles(
                                    &mut pos_cc.0,
                                    &mut pos_cr.0,
                                    &mut pos_bc.0,
                                    &mut pos_br.0,
                                    height_map,
                                );
                            }

                            if (ground_cc == ground_bc
                                && ground_cr == ground_br
                                && ground_cc != ground_cr
                                && ground_bc != ground_br)
                            {
                                let height: Vec<f32> = ramp_map(points).collect();
                                let height_map = vec![
                                    rotate(height.clone(), points, 0),
                                    rotate(height.clone(), points, 2),
                                    rotate(height.clone(), points, 0),
                                    rotate(height.clone(), points, 2),
                                ];

                                stitch_tiles(
                                    &mut pos_cc.0,
                                    &mut pos_cr.0,
                                    &mut pos_bc.0,
                                    &mut pos_br.0,
                                    height_map,
                                );
                            }
                            set_mesh_position(&pos_cc.0, &pos_cc.1, &mut meshes);
                            set_mesh_position(&pos_cr.0, &pos_cr.1, &mut meshes);
                            set_mesh_position(&pos_bc.0, &pos_bc.1, &mut meshes);
                            set_mesh_position(&pos_br.0, &pos_br.1, &mut meshes);
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
    let point = 1.0;

    for z in 0..half {
        for x in 0..half {
            let base_1 = (z * points + x) as usize;
            let base_2 = ((z + 1) * points + x) as usize;

            let top_left = height_map[0][base_1 + 0];
            let bot_left = height_map[0][base_2 + 0];

            let top_right = height_map[0][base_1 + 1];
            let bot_right = height_map[0][base_2 + 1];

            let i = (((z + half) * sub_quads + (x + half)) * 6) as usize;
            tile1[i + 0][1] = lerp(top_left, tile1[i + 0][1], point);
            tile1[i + 1][1] = lerp(bot_left, tile1[i + 1][1], point);
            tile1[i + 2][1] = lerp(top_right, tile1[i + 2][1], point);
            tile1[i + 3][1] = lerp(top_right, tile1[i + 3][1], point);
            tile1[i + 4][1] = lerp(bot_left, tile1[i + 4][1], point);
            tile1[i + 5][1] = lerp(bot_right, tile1[i + 5][1], point);

            let top_left = height_map[1][base_1 + 0];
            let bot_left = height_map[1][base_2 + 0];

            let top_right = height_map[1][base_1 + 1];
            let bot_right = height_map[1][base_2 + 1];

            let i = (((z + half) * sub_quads + x) * 6) as usize;
            tile2[i + 0][1] = lerp(top_left, tile2[i + 0][1], point);
            tile2[i + 1][1] = lerp(bot_left, tile2[i + 1][1], point);
            tile2[i + 2][1] = lerp(top_right, tile2[i + 2][1], point);
            tile2[i + 3][1] = lerp(top_right, tile2[i + 3][1], point);
            tile2[i + 4][1] = lerp(bot_left, tile2[i + 4][1], point);
            tile2[i + 5][1] = lerp(bot_right, tile2[i + 5][1], point);

            let top_left = height_map[2][base_1 + 0];
            let bot_left = height_map[2][base_2 + 0];

            let top_right = height_map[2][base_1 + 1];
            let bot_right = height_map[2][base_2 + 1];

            let i = ((z * sub_quads + (x + half)) * 6) as usize;
            tile3[i + 0][1] = lerp(top_left, tile3[i + 0][1], point);
            tile3[i + 1][1] = lerp(bot_left, tile3[i + 1][1], point);
            tile3[i + 2][1] = lerp(top_right, tile3[i + 2][1], point);
            tile3[i + 3][1] = lerp(top_right, tile3[i + 3][1], point);
            tile3[i + 4][1] = lerp(bot_left, tile3[i + 4][1], point);
            tile3[i + 5][1] = lerp(bot_right, tile3[i + 5][1], point);

            let top_left = height_map[3][base_1 + 0];
            let bot_left = height_map[3][base_2 + 0];

            let top_right = height_map[3][base_1 + 1];
            let bot_right = height_map[3][base_2 + 1];

            let i = ((z * sub_quads + x) * 6) as usize;
            tile4[i + 0][1] = lerp(top_left, tile4[i + 0][1], point);
            tile4[i + 1][1] = lerp(bot_left, tile4[i + 1][1], point);
            tile4[i + 2][1] = lerp(top_right, tile4[i + 2][1], point);
            tile4[i + 3][1] = lerp(top_right, tile4[i + 3][1], point);
            tile4[i + 4][1] = lerp(bot_left, tile4[i + 4][1], point);
            tile4[i + 5][1] = lerp(bot_right, tile4[i + 5][1], point);
        }
    }
}

fn rotate<T: Copy>(array: Vec<T>, size: i32, step: i32) -> Vec<T> {
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
    x * x
    // let x2 = x * x;
    // let x4 = x2 * x2;
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
