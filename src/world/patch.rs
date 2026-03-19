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

                        if let (
                            Some(mut tile_1),
                            Some(mut tile_2),
                            Some(mut tile_3),
                            Some(mut tile_4),
                        ) = (
                            tile_mesh_positions(&world, tile_cc, &query, &meshes),
                            tile_mesh_positions(&world, tile_cr, &query, &meshes),
                            tile_mesh_positions(&world, tile_bc, &query, &meshes),
                            tile_mesh_positions(&world, tile_br, &query, &meshes),
                        ) {
                            stitch_tiles(
                                &mut tile_1.0,
                                &mut tile_2.0,
                                &mut tile_3.0,
                                &mut tile_4.0,
                            );
                            set_mesh_position(&tile_1.0, &tile_1.1, &mut meshes);
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
) {
    let sub_quads = 2i32.pow(4.0 as u32);

    let half = sub_quads / 2;
    let points = half + 1;

    let point = 1.0;

    let height: Vec<f32> = curve_map(points).collect();
    let height = rotate(&height, points, 2);

    for z in 0..half {
        for x in 0..half {
            let i = (((z + half) * sub_quads + (x + half)) * 6) as usize;

            let base_1 = (z * points + x) as usize;
            let base_2 = ((z + 1) * points + x) as usize;

            let top_left = height[base_1 + 0];
            let bot_left = height[base_2 + 0];

            let top_right = height[base_1 + 1];
            let bot_right = height[base_2 + 1];

            tile1[i + 0][1] = lerp(top_left, tile1[i + 0][1], point);
            tile1[i + 1][1] = lerp(bot_left, tile1[i + 1][1], point);
            tile1[i + 2][1] = lerp(top_right, tile1[i + 2][1], point);
            tile1[i + 3][1] = lerp(top_right, tile1[i + 3][1], point);
            tile1[i + 4][1] = lerp(bot_left, tile1[i + 4][1], point);
            tile1[i + 5][1] = lerp(bot_right, tile1[i + 5][1], point);
        }
    }
}

fn rotate<T: Copy>(array: &[T], size: i32, step: i32) -> Vec<T> {
    let mut rotated = Vec::with_capacity((size * size) as usize);

    for y in 0..size {
        for x in 0..size {
            let i = match step {
                0 => 0,
                1 => (size - 1 - x) * size + y,
                2 => (size - y) * size - 1 - x,
                3 => (size - 1 - y) + size * x,
                _ => panic!("Unreachable step: {}", step)
            };
            rotated.push(array[i as usize]);
        }
    }

    rotated
}


fn relu(v: f32) -> f32 {
    if v > 0.0 { v } else { 0.0 }
}

fn ramp_map(size: i32) -> impl Iterator<Item = f32> {
    (0..size * size)
        .map(move |i| ((i % size) * (i % size)) as f32 / ((size - 1) * (size - 1)) as f32)
}

fn curve_map(size: i32) -> impl Iterator<Item = f32> {
    (0..size * size)
        .map(move |i| ((i % size) * (i / size)) as f32 / ((size - 1) * (size - 1)) as f32)
}

fn bend_corner(tile: &mut [[f32; 3]], points: [f32; 3], size: usize) {
    for z in 0..size {
        for x in 0..size {
            if x + z < size - 1 {
                continue;
            }
            let i = (((z + size) * (size * 2) + (x + size)) * 6) as usize;

            let mut scale_z = ((x + z - 2) - 7) as f32 / (14 - 7) as f32;
            let scale_0 = ((x + z) - 7) as f32 / (14 - 7) as f32;

            if x + z == 7 {
                scale_z = scale_0;
            }
            let mut scale_q = (scale_z + scale_0) / 2.0;
            if x + z == 8 {
                scale_z = 0.0;
                scale_q = 0.0;
            }

            tile[i + 0][1] = lerp(scale_z, tile[i + 0][1], points[1]);
            tile[i + 1][1] = lerp(scale_q, tile[i + 1][1], points[1]);
            tile[i + 2][1] = lerp(scale_q, tile[i + 2][1], points[1]);
            tile[i + 3][1] = lerp(scale_q, tile[i + 3][1], points[1]);
            tile[i + 4][1] = lerp(scale_q, tile[i + 4][1], points[1]);
            tile[i + 5][1] = lerp(scale_0, tile[i + 5][1], points[1]);
        }
    }
}

#[inline]
fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - ((-2.0 * x + 2.0) as f32).powf(3.0) / 2.0
    }
}

fn ease_in_out_sine(x: f32) -> f32 {
    -(f32::cos(std::f32::consts::PI * x) - 1.0) / 2.0
}

pub fn ease_in_out_circ(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

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
