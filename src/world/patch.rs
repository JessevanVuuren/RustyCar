use crate::{
    extra::math::{ease_in_quint, lerp, s_curve},
    world::components::{
        Comp, Grass, GrassConfig, Model, Offset, Placement, Range, Rotation, StaticWorld,
        TILE_SIZE, TilePos, TileType, TileWorld, Value,
    },
};
use bevy::{
    mesh::VertexAttributeValues, prelude::*,
    render::render_resource::encase::vector::AsMutVectorParts,
};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::{collections::HashMap, f32::consts::FRAC_PI_2, iter, ops::Mul};

enum Stitch {
    Horizontal,
    Vertical,
    Corner,
    TSplit,
    XSplit,
}

struct PatchWorks {
    template: [usize; 4],
    stitch: Stitch,
}

const MID_POINT: f32 = 0.25;

const CONFIGURATION: [PatchWorks; 4] = [
    PatchWorks {
        stitch: Stitch::Horizontal,
        template: [1, 1, 2, 2],
    },
    PatchWorks {
        stitch: Stitch::Corner,
        template: [1, 1, 2, 1],
    },
    PatchWorks {
        stitch: Stitch::TSplit,
        template: [2, 1, 3, 1],
    },
    PatchWorks {
        stitch: Stitch::XSplit,
        template: [1, 2, 3, 4],
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
                        println!("");
                        println!("Tile");
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
                            let points = sub_quads + 1;

                            let observed = [ground_tl, ground_tr, ground_bl, ground_br];

                            for conf in CONFIGURATION {
                                for i in 0..4 {
                                    let scaler: Vec<f32> = match conf.stitch {
                                        Stitch::Horizontal => {
                                            let ramp: Vec<f32> =
                                                smooth_bump(points, 2.0, 0.4).collect();
                                            rotate(&ramp, points, 1)
                                        }
                                        Stitch::Vertical => smooth_bump(points, 2.0, 0.4).collect(),
                                        Stitch::Corner => smooth_corner(points, 2.0, 0.4).collect(),
                                        Stitch::TSplit => smooth_tsplit(points, 2.0, 0.4).collect(),
                                        Stitch::XSplit => smooth_xsplit(points, 2.0, 0.4).collect(),
                                    };

                                    let mut result = true;
                                    let highest = core_highest_value(&conf.template);
                                    let mut temp = core_rotate(&conf.template, i);

                                    for _ in 0..highest {
                                        let ones = core_isolate_ones(&temp);
                                        let multi = core_multiply(&ones, &observed);
                                        let uniq = core_highest_value(&multi);

                                        let inv = core_inverse(&ones);
                                        let inv_multi = core_multiply(&inv, &observed);

                                        let res_1 = core_continuity(&multi);
                                        let res_2 = core_contains(&inv_multi, &vec![uniq]);

                                        println!(
                                            "{:?}, {:?}, {:?}, {:?}, {}, {}",
                                            ones, multi, temp, observed, !res_1, res_2
                                        );

                                        if !res_1 || res_2 {
                                            result = false;
                                        }
                                        temp = core_lower_index(&temp);
                                    }

                                    if result {
                                        let (horizontal, vertical) = create_height_map(
                                            &mut pos_tl.0,
                                            &mut pos_tr.0,
                                            &mut pos_bl.0,
                                            &mut pos_br.0,
                                        );

                                        let scaler = rotate(&scaler, points, i);
                                        let mut height_map = map_averages(&horizontal, &vertical);

                                        stitch_tiles(
                                            &mut pos_tl.0,
                                            &mut pos_tr.0,
                                            &mut pos_bl.0,
                                            &mut pos_br.0,
                                            height_map,
                                            scaler,
                                        );

                                        set_mesh_position(&pos_tl.0, &pos_tl.1, &mut meshes);
                                        set_mesh_position(&pos_tr.0, &pos_tr.1, &mut meshes);
                                        set_mesh_position(&pos_bl.0, &pos_bl.1, &mut meshes);
                                        set_mesh_position(&pos_br.0, &pos_br.1, &mut meshes);

                                        break;
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

fn map_averages(horizontal: &Vec<f32>, vertical: &Vec<f32>) -> Vec<f32> {
    let size = horizontal.len();
    let sqrt = (size as f32).sqrt() as usize;
    let mut average = Vec::with_capacity(size);

    for i in 0..size {
        let x = i % sqrt;
        let z = i / sqrt;
        let i2 = x * sqrt + z;

        average.push((horizontal[i] + vertical[i2]) / 2.0);
    }

    average
}

fn create_height_map(
    tile1: &mut [[f32; 3]],
    tile2: &mut [[f32; 3]],
    tile3: &mut [[f32; 3]],
    tile4: &mut [[f32; 3]],
) -> (Vec<f32>, Vec<f32>) {
    let sub_quads = 2i32.pow(4.0 as u32);
    let half = sub_quads / 2;
    let quad_points = 6;
    let row = sub_quads * quad_points;
    let half_row = row / 2;

    let mut points_horizontal = Vec::new();
    let mut points_vertical = Vec::new();

    for i in 0..half {
        let index_1 = ((i + half + 1) * row - half_row) as usize;
        let index_2 = ((i + half + 1) * row - half_row - quad_points) as usize;

        let start = tile1[index_1 + 0][1];
        let stop = tile2[index_2 + 2][1];
        points_horizontal.append(&mut points_along_line(start, stop, sub_quads));
    }

    for i in 0..half {
        let index_1 = ((i + 1) * row - half_row) as usize;
        let index_2 = ((i + 1) * row - half_row - quad_points) as usize;

        let start = tile3[index_1 + 0][1];
        let stop = tile4[index_2 + 2][1];

        points_horizontal.append(&mut points_along_line(start, stop, sub_quads));

        if i == half - 1 {
            let start = tile3[index_1 + 1][1];
            let stop = tile4[index_2 + 5][1];

            points_horizontal.append(&mut points_along_line(start, stop, sub_quads));
        }
    }

    for i in 0..half {
        let index_1 = (row * half + i * quad_points + half_row) as usize;
        let index_2 = (row * (half - 1) + i * quad_points + half_row) as usize;

        let start = tile1[index_1 + 0][1];
        let stop = tile3[index_2 + 1][1];

        points_vertical.append(&mut points_along_line(start, stop, sub_quads));
    }

    for i in 0..half {
        let index_1 = (row * half + i * quad_points) as usize;
        let index_2 = (row * (half - 1) + i * quad_points) as usize;

        let start = tile2[index_1 + 0][1];
        let stop = tile4[index_2 + 1][1];

        points_vertical.append(&mut points_along_line(start, stop, sub_quads));

        if i == half - 1 {
            let start = tile2[index_1 + 2][1];
            let stop = tile4[index_2 + 5][1];

            points_vertical.append(&mut points_along_line(start, stop, sub_quads));
        }
    }

    (points_horizontal, points_vertical)
}

fn points_along_line(start: f32, stop: f32, points: i32) -> Vec<f32> {
    let mut data = Vec::with_capacity(points as usize + 1);

    for i in 0..=points {
        let norm = i as f32 / points as f32;
        data.push(lerp(norm, start, stop));
    }

    data
}

fn stitch_tiles(
    tile1: &mut [[f32; 3]],
    tile2: &mut [[f32; 3]],
    tile3: &mut [[f32; 3]],
    tile4: &mut [[f32; 3]],
    height_map: Vec<f32>,
    scaler: Vec<f32>,
) {
    let sub_quads = 2i32.pow(4.0 as u32);
    let half = sub_quads / 2;
    let points = sub_quads + 1;

    for z in 0..half {
        for x in 0..half {
            let base_1 = (z * points + x) as usize;
            let base_2 = ((z + 1) * points + x) as usize;

            let top_left = height_map[base_1 + 0];
            let bot_left = height_map[base_2 + 0];

            let top_right = height_map[base_1 + 1];
            let bot_right = height_map[base_2 + 1];

            let top_left_s = scaler[base_1 + 0];
            let bot_left_s = scaler[base_2 + 0];

            let top_right_s = scaler[base_1 + 1];
            let bot_right_s = scaler[base_2 + 1];

            let i = (((z + half) * sub_quads + (x + half)) * 6) as usize;
            tile1[i + 0][1] = lerp(top_left_s, tile1[i + 0][1], top_left);
            tile1[i + 1][1] = lerp(bot_left_s, tile1[i + 1][1], bot_left);
            tile1[i + 2][1] = lerp(top_right_s, tile1[i + 2][1], top_right);
            tile1[i + 3][1] = lerp(top_right_s, tile1[i + 3][1], top_right);
            tile1[i + 4][1] = lerp(bot_left_s, tile1[i + 4][1], bot_left);
            tile1[i + 5][1] = lerp(bot_right_s, tile1[i + 5][1], bot_right);

            let base_1 = (z * points + x + half) as usize;
            let base_2 = ((z + 1) * points + x + half) as usize;

            let top_left = height_map[base_1 + 0];
            let bot_left = height_map[base_2 + 0];

            let top_right = height_map[base_1 + 1];
            let bot_right = height_map[base_2 + 1];

            let top_left_s = scaler[base_1 + 0];
            let bot_left_s = scaler[base_2 + 0];

            let top_right_s = scaler[base_1 + 1];
            let bot_right_s = scaler[base_2 + 1];

            let i = (((z + half) * sub_quads + x) * 6) as usize;
            tile2[i + 0][1] = lerp(top_left_s, tile2[i + 0][1], top_left);
            tile2[i + 1][1] = lerp(bot_left_s, tile2[i + 1][1], bot_left);
            tile2[i + 2][1] = lerp(top_right_s, tile2[i + 2][1], top_right);
            tile2[i + 3][1] = lerp(top_right_s, tile2[i + 3][1], top_right);
            tile2[i + 4][1] = lerp(bot_left_s, tile2[i + 4][1], bot_left);
            tile2[i + 5][1] = lerp(bot_right_s, tile2[i + 5][1], bot_right);

            let base_1 = ((z + half) * points + x) as usize;
            let base_2 = (((z + half) + 1) * points + x) as usize;

            let top_left = height_map[base_1 + 0];
            let bot_left = height_map[base_2 + 0];

            let top_right = height_map[base_1 + 1];
            let bot_right = height_map[base_2 + 1];

            let top_left_s = scaler[base_1 + 0];
            let bot_left_s = scaler[base_2 + 0];

            let top_right_s = scaler[base_1 + 1];
            let bot_right_s = scaler[base_2 + 1];

            let i = ((z * sub_quads + (x + half)) * 6) as usize;
            tile3[i + 0][1] = lerp(top_left_s, tile3[i + 0][1], top_left);
            tile3[i + 1][1] = lerp(bot_left_s, tile3[i + 1][1], bot_left);
            tile3[i + 2][1] = lerp(top_right_s, tile3[i + 2][1], top_right);
            tile3[i + 3][1] = lerp(top_right_s, tile3[i + 3][1], top_right);
            tile3[i + 4][1] = lerp(bot_left_s, tile3[i + 4][1], bot_left);
            tile3[i + 5][1] = lerp(bot_right_s, tile3[i + 5][1], bot_right);

            let base_1 = ((z + half) * points + x + half) as usize;
            let base_2 = (((z + half) + 1) * points + x + half) as usize;

            let top_left = height_map[base_1 + 0];
            let bot_left = height_map[base_2 + 0];

            let top_right = height_map[base_1 + 1];
            let bot_right = height_map[base_2 + 1];

            let top_left_s = scaler[base_1 + 0];
            let bot_left_s = scaler[base_2 + 0];

            let top_right_s = scaler[base_1 + 1];
            let bot_right_s = scaler[base_2 + 1];

            let i = ((z * sub_quads + x) * 6) as usize;
            tile4[i + 0][1] = lerp(top_left_s, tile4[i + 0][1], top_left);
            tile4[i + 1][1] = lerp(bot_left_s, tile4[i + 1][1], bot_left);
            tile4[i + 2][1] = lerp(top_right_s, tile4[i + 2][1], top_right);
            tile4[i + 3][1] = lerp(top_right_s, tile4[i + 3][1], top_right);
            tile4[i + 4][1] = lerp(bot_left_s, tile4[i + 4][1], bot_left);
            tile4[i + 5][1] = lerp(bot_right_s, tile4[i + 5][1], bot_right);
        }
    }
}

fn core_isolate_ones(core: &[usize; 4]) -> [usize; 4] {
    [
        if core[0] == 1 { 1 } else { 0 },
        if core[1] == 1 { 1 } else { 0 },
        if core[2] == 1 { 1 } else { 0 },
        if core[3] == 1 { 1 } else { 0 },
    ]
}

fn core_lower_index(core: &[usize; 4]) -> [usize; 4] {
    [
        if core[0] > 1 { core[0] - 1 } else { 0 },
        if core[1] > 1 { core[1] - 1 } else { 0 },
        if core[2] > 1 { core[2] - 1 } else { 0 },
        if core[3] > 1 { core[3] - 1 } else { 0 },
    ]
}

fn core_multiply(core: &[usize; 4], observed: &[usize; 4]) -> [usize; 4] {
    [
        core[0].clone() * observed[0].clone(),
        core[1].clone() * observed[1].clone(),
        core[2].clone() * observed[2].clone(),
        core[3].clone() * observed[3].clone(),
    ]
}

fn core_highest_value(core: &[usize; 4]) -> usize {
    let mut highest = core[0];

    if core[1] > highest {
        highest = core[1]
    }
    if core[2] > highest {
        highest = core[2]
    }
    if core[3] > highest {
        highest = core[3]
    }

    highest
}

fn core_continuity(core: &[usize; 4]) -> bool {
    let mut base = 0 as usize;

    for &x in core {
        if base == 0 && x != 0 {
            base = x;
        }

        if x != 0 && x != base {
            return false;
        }
    }

    true
}

fn core_inverse(core: &[usize; 4]) -> [usize; 4] {
    core.map(|i| if i == 0 { 1 } else { 0 })
}

fn core_rotate(core: &[usize; 4], step: usize) -> [usize; 4] {
    match step % 4 {
        0 => *core,
        1 => [core[2], core[0], core[3], core[1]],
        2 => [core[3], core[2], core[1], core[0]],
        3 => [core[1], core[3], core[0], core[2]],
        _ => unreachable!(),
    }
}

fn core_contains(core: &[usize; 4], numbers: &Vec<usize>) -> bool {
    let mut flag = false;

    for &number in numbers {
        if number == core[0] || number == core[1] || number == core[2] || number == core[3] {
            flag = true;
        }
    }

    flag
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

fn smooth_xsplit(size: i32, intensity: f32, spread: f32) -> impl Iterator<Item = f32> {
    let vertical: Vec<f32> = smooth_bump(size, 2.0, 0.4).collect();
    let horizontal: Vec<f32> = rotate(&vertical, size, 1);

    (0..size * size).map(move |i| {
        let x = i % size;
        let y = i / size;

        horizontal[i as usize].max(vertical[i as usize])
    })
}

fn smooth_tsplit(size: i32, intensity: f32, spread: f32) -> impl Iterator<Item = f32> {
    let vertical: Vec<f32> = smooth_bump(size, 2.0, 0.4).collect();
    let horizontal: Vec<f32> = rotate(&vertical, size, 1);

    let half = size / 2;

    (0..size * size).map(move |i| {
        let x = i % size;
        let y = i / size;

        if x > half {
            vertical[i as usize]
        } else {
            horizontal[i as usize].max(vertical[i as usize])
        }
    })
}

fn smooth_corner(size: i32, intensity: f32, spread: f32) -> impl Iterator<Item = f32> {
    let vertical: Vec<f32> = smooth_bump(size, 2.0, 0.4).collect();
    let horizontal: Vec<f32> = rotate(&vertical, size, 1);

    let half = size / 2;

    (0..size * size).map(move |i| {
        let x = i % size;
        let y = i / size;

        if x < half && y < half {
            horizontal[i as usize]
        } else if x > half && y > half {
            vertical[i as usize]
        } else if x >= half && y <= half {
            vertical[i as usize].min(horizontal[i as usize])
        } else {
            vertical[i as usize].max(horizontal[i as usize])
        }
    })
}

fn smooth_bump(size: i32, intensity: f32, spread: f32) -> impl Iterator<Item = f32> {
    let spread = (size as f32 * spread) as i32 / 2;
    let half = size / 2;
    (0..size * size).map(move |i| {
        let x = i % size;
        let y = i / size;

        if x < spread {
            0.0
        } else if x < half {
            let n = (x - spread) as f32 / (half - spread) as f32;
            s_curve(n, 1.0 + intensity).clamp(0.0, 1.0)
        } else if x < size - spread {
            let n = (x - half) as f32 / (half - spread) as f32;
            s_curve(n, -1.0 + (-1.0 * intensity)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    })
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
