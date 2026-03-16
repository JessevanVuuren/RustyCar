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

                        if let (Some(handle_1), Some(handle_2)) = (
                            mesh_on_tile(&world, tile, &query),
                            mesh_on_tile(&world, tile_right, &query),
                        ) {
                            if let (Some(mut pos_1), Some(mut pos_2)) = (
                                get_mesh_positions(&meshes, &handle_1),
                                get_mesh_positions(&meshes, &handle_2),
                            ) {
                                blend_tiles1(&mut pos_1, &pos_2, config.subdivisions);
                                blend_tiles2(&mut pos_2, &pos_1, config.subdivisions);

                                // blend_tiles_test(&mut pos_1, &mut pos_2, config.subdivisions);

                                set_mesh_position(pos_1, &handle_1, &mut meshes);
                                set_mesh_position(pos_2, &handle_2, &mut meshes);
                            }
                        };
                    }
                }
            }
            _ => (),
        }
    }
}
// 8 - 3 * .5

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

fn blend_tiles1(points_1: &mut [[f32; 3]], points_2: &[[f32; 3]], subdivision: u8) {
    let sub_quads = 2u32.pow(subdivision as u32);
    let row_step = sub_quads * 6;

    for z in 0..sub_quads {
        let y0 = points_1[(z * row_step) as usize][1];
        let y1 = points_2[((z + 1) * row_step - 3) as usize][1];
        let mid_point_1 = (y0 + y1) / 2.0;

        let y0 = points_1[(z * row_step + 1) as usize][1];
        let y1 = points_2[((z + 1) * row_step - 1) as usize][1];
        let mid_point_2 = (y0 + y1) / 2.0;

        for x in 0..sub_quads {
            let i = (z * sub_quads + x) as usize;
            let scale_1 = (sub_quads - x) as f32 / sub_quads as f32;
            let scale_2 = (sub_quads - (x + 1)) as f32 / sub_quads as f32;

            points_1[i * 6 + 0][1] = lerp(
                ease_in_out_cubic(scale_1),
                mid_point_1,
                points_1[i * 6 + 0][1],
            );
            points_1[i * 6 + 1][1] = lerp(
                ease_in_out_cubic(scale_1),
                mid_point_2,
                points_1[i * 6 + 1][1],
            );
            points_1[i * 6 + 2][1] = lerp(
                ease_in_out_cubic(scale_2),
                mid_point_1,
                points_1[i * 6 + 2][1],
            );
            points_1[i * 6 + 3][1] = lerp(
                ease_in_out_cubic(scale_2),
                mid_point_1,
                points_1[i * 6 + 3][1],
            );
            points_1[i * 6 + 4][1] = lerp(
                ease_in_out_cubic(scale_1),
                mid_point_2,
                points_1[i * 6 + 4][1],
            );
            points_1[i * 6 + 5][1] = lerp(
                ease_in_out_cubic(scale_2),
                mid_point_2,
                points_1[i * 6 + 5][1],
            );
        }
    }
}

fn blend_tiles2(points_1: &mut [[f32; 3]], points_2: &[[f32; 3]], subdivision: u8) {
    let sub_quads = 2u32.pow(subdivision as u32);
    let row_step = sub_quads * 6;

    for z in 0..sub_quads {
        let y0 = points_2[(z * row_step) as usize][1];
        let y1 = points_1[((z + 1) * row_step - 3) as usize][1];
        let mid_point_1 = (y0 + y1) / 2.0;

        let y0 = points_2[(z * row_step + 1) as usize][1];
        let y1 = points_1[((z + 1) * row_step - 1) as usize][1];
        let mid_point_2 = (y0 + y1) / 2.0;

        for x in 0..sub_quads {
            let i = (z * sub_quads + x) as usize;
            let scale_1 = (sub_quads - x) as f32 / sub_quads as f32;
            let scale_2 = (sub_quads - (x + 1)) as f32 / sub_quads as f32;

            points_1[i * 6 + 0][1] = lerp(ease_in_out_cubic(scale_1), points_1[i * 6 + 0][1], mid_point_1);
            points_1[i * 6 + 1][1] = lerp(ease_in_out_cubic(scale_1), points_1[i * 6 + 1][1], mid_point_2);
            points_1[i * 6 + 2][1] = lerp(ease_in_out_cubic(scale_2), points_1[i * 6 + 2][1], mid_point_1);
            points_1[i * 6 + 3][1] = lerp(ease_in_out_cubic(scale_2), points_1[i * 6 + 3][1], mid_point_1);
            points_1[i * 6 + 4][1] = lerp(ease_in_out_cubic(scale_1), points_1[i * 6 + 4][1], mid_point_2);
            points_1[i * 6 + 5][1] = lerp(ease_in_out_cubic(scale_2), points_1[i * 6 + 5][1], mid_point_2);
        }
    }
}

fn set_mesh_position(positions: Vec<[f32; 3]>, handle: &Handle<Mesh>, meshes: &mut Assets<Mesh>) {
    if let Some(mesh) = meshes.get_mut(handle) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
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
        if let Ok(mesh3d) = query.get(t.entity()) {
            return Some(mesh3d.0.clone());
        }
    }

    None
}
