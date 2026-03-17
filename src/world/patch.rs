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
                        let mut tile_right = TilePos::new(tile.x + 1, tile.z);
                        let mut tile_down = TilePos::new(tile.x, tile.z + 1);

                        let Some(ground) = world.ground.get(&tile) else {
                            continue;
                        };

                        let Some(handle) = mesh_on_tile(&world, tile, &query) else {
                            continue;
                        };

                        let Some(mut pos) = get_mesh_positions(&meshes, &handle) else {
                            continue;
                        };

                        if let Some(ground_right) = world.ground.get(&tile_right) {
                            if ground.id != ground_right.id {
                                if let Some(handle_right) = mesh_on_tile(&world, tile_right, &query)
                                {
                                    if let Some(mut pos_right) =
                                        get_mesh_positions(&meshes, &handle_right)
                                    {
                                        blend_tiles1(&mut pos, &pos_right, config.subdivisions);
                                        blend_tiles2(&mut pos_right, &pos, config.subdivisions);

                                        set_mesh_position(&pos, &handle, &mut meshes);
                                        set_mesh_position(&pos_right, &handle_right, &mut meshes);
                                    }
                                }
                            }
                        }

                        if let Some(ground_down) = world.ground.get(&tile_down) {
                            if ground.id != ground_down.id {
                                if let Some(handle_down) = mesh_on_tile(&world, tile_down, &query) {
                                    if let Some(mut pos_down) =
                                        get_mesh_positions(&meshes, &handle_down)
                                    {
                                        blend_tiles3(&mut pos, &pos_down, config.subdivisions);
                                        blend_tiles4(&mut pos_down, &pos, config.subdivisions);

                                        set_mesh_position(&pos, &handle, &mut meshes);
                                        set_mesh_position(&pos_down, &handle_down, &mut meshes);
                                    }
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
    let x2 = x * x;
    let x4 = x2 * x2;
    let x8 = x4 * x4;
    let x16 = x8 * x8;
    x16 * x16
}

fn blend_tiles1(points_1: &mut [[f32; 3]], points_2: &[[f32; 3]], subdivision: u8) {
    let sub_quads = 2i32.pow(subdivision as u32);
    let row_step = sub_quads * 6;
    let skip = 10;

    for z in 0..sub_quads {
        let y0 = points_1[(z * row_step) as usize][1];
        let y1 = points_2[((z + 1) * row_step - 3) as usize][1];
        let mid_point_1 = (y0 + y1) / 2.0;

        let y0 = points_1[(z * row_step + 1) as usize][1];
        let y1 = points_2[((z + 1) * row_step - 1) as usize][1];
        let mid_point_2 = (y0 + y1) / 2.0;

        for x in skip..sub_quads {
            let i = ((z * sub_quads + x) * 6) as usize;

            let scale_1 = (x - skip) as f32 / (sub_quads - skip) as f32;
            let scale_2 = ((x + 1) - skip) as f32 / (sub_quads - skip) as f32;

            // let scale_1 = ease_in_quint(scale_1);
            // let scale_2 = ease_in_quint(scale_2);

            points_1[i + 0][1] = lerp(scale_1, points_1[i + 0][1], mid_point_1);
            points_1[i + 1][1] = lerp(scale_1, points_1[i + 1][1], mid_point_2);
            points_1[i + 2][1] = lerp(scale_2, points_1[i + 2][1], mid_point_1);
            points_1[i + 3][1] = lerp(scale_2, points_1[i + 3][1], mid_point_1);
            points_1[i + 4][1] = lerp(scale_1, points_1[i + 4][1], mid_point_2);
            points_1[i + 5][1] = lerp(scale_2, points_1[i + 5][1], mid_point_2);
        }
    }
}

fn blend_tiles2(points_1: &mut [[f32; 3]], points_2: &[[f32; 3]], subdivision: u8) {
    let sub_quads = 2i32.pow(subdivision as u32);
    let row_step = sub_quads * 6;
    let skip = 10;

    for z in 0..sub_quads {
        let y0 = points_2[(z * row_step) as usize][1];
        let y1 = points_1[((z + 1) * row_step - 3) as usize][1];
        let mid_point_1 = (y0 + y1) / 2.0;

        let y0 = points_2[(z * row_step + 1) as usize][1];
        let y1 = points_1[((z + 1) * row_step - 1) as usize][1];
        let mid_point_2 = (y0 + y1) / 2.0;

        for x in 0..sub_quads - skip {
            let i = ((z * sub_quads + x) * 6) as usize;

            let scale_1 = 1.0 - (x - 0) as f32 / (sub_quads - skip - 0) as f32;
            let scale_2 = 1.0 - ((x + 1) - 0) as f32 / (sub_quads - skip - 0) as f32;

            // let scale_1 = ease_in_quint(scale_1);
            // let scale_2 = ease_in_quint(scale_2);

            points_1[i + 0][1] = lerp(scale_1, points_1[i + 0][1], mid_point_1);
            points_1[i + 1][1] = lerp(scale_1, points_1[i + 1][1], mid_point_2);
            points_1[i + 2][1] = lerp(scale_2, points_1[i + 2][1], mid_point_1);
            points_1[i + 3][1] = lerp(scale_2, points_1[i + 3][1], mid_point_1);
            points_1[i + 4][1] = lerp(scale_1, points_1[i + 4][1], mid_point_2);
            points_1[i + 5][1] = lerp(scale_2, points_1[i + 5][1], mid_point_2);
        }
    }
}

fn blend_tiles3(points_1: &mut [[f32; 3]], points_2: &[[f32; 3]], subdivision: u8) {
    let sub_quads = 2i32.pow(subdivision as u32);
    let row_step = sub_quads * 6;
    let skip = 10;

    for z in 0..sub_quads {
        let y0 = points_1[(z * 6) as usize][1];
        let y1 = points_2[(z * 6 + (row_step * (sub_quads - 1)) + 4) as usize][1];
        let mid_point_1 = (y0 + y1) / 2.0;

        let y0 = points_1[(z * 6 + 2) as usize][1];
        let y1 = points_2[(z * 6 + (row_step * (sub_quads - 1)) + 5) as usize][1];
        let mid_point_2 = (y0 + y1) / 2.0;

        for x in skip..sub_quads {
            let i = ((x * sub_quads + z) * 6) as usize;

            let scale_1 = (x - skip) as f32 / (sub_quads - skip) as f32;
            let scale_2 = ((x + 1) - skip) as f32 / (sub_quads - skip) as f32;

            // let scale_1 = ease_in_quint(scale_1);
            // let scale_2 = ease_in_quint(scale_2);

            points_1[i + 0][1] = lerp(scale_1, points_1[i + 0][1], mid_point_1);
            points_1[i + 1][1] = lerp(scale_2, points_1[i + 1][1], mid_point_1);
            points_1[i + 2][1] = lerp(scale_1, points_1[i + 2][1], mid_point_2);
            points_1[i + 3][1] = lerp(scale_1, points_1[i + 3][1], mid_point_2);
            points_1[i + 4][1] = lerp(scale_2, points_1[i + 4][1], mid_point_1);
            points_1[i + 5][1] = lerp(scale_2, points_1[i + 5][1], mid_point_2);
        }
    }
}

fn blend_tiles4(points_1: &mut [[f32; 3]], points_2: &[[f32; 3]], subdivision: u8) {
    let sub_quads = 2i32.pow(subdivision as u32);
    let row_step = sub_quads * 6;
    let skip = 10;

    for z in 0..sub_quads {
        let y0 = points_2[(z * 6) as usize][1];
        let y1 = points_1[(z * 6 + (row_step * (sub_quads - 1)) + 4) as usize][1];
        let mid_point_1 = (y0 + y1) / 2.0;

        let y0 = points_2[(z * 6 + 2) as usize][1];
        let y1 = points_1[(z * 6 + (row_step * (sub_quads - 1)) + 5) as usize][1];
        let mid_point_2 = (y0 + y1) / 2.0;

        for x in 0..sub_quads - skip {
            let i = ((x * sub_quads + z) * 6) as usize;

            let scale_1 = 1.0 - x as f32 / (sub_quads - skip) as f32;
            let scale_2 = 1.0 - (x + 1) as f32 / (sub_quads - skip) as f32;

            // let scale_1 = ease_in_quint(scale_1);
            // let scale_2 = ease_in_quint(scale_2);

            points_1[i + 0][1] = lerp(scale_1, points_1[i + 0][1], mid_point_1);
            points_1[i + 1][1] = lerp(scale_2, points_1[i + 1][1], mid_point_1);
            points_1[i + 2][1] = lerp(scale_1, points_1[i + 2][1], mid_point_2);
            points_1[i + 3][1] = lerp(scale_1, points_1[i + 3][1], mid_point_2);
            points_1[i + 4][1] = lerp(scale_2, points_1[i + 4][1], mid_point_1);
            points_1[i + 5][1] = lerp(scale_2, points_1[i + 5][1], mid_point_2);
        }
    }
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
