use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use rand::{RngExt, rngs::SmallRng};

use crate::{
    extra::noise::perlin_2d,
    world::components::{GroundConfig, Noise, TILE_SIZE},
};

fn rgb_lerp(c1: Color, c2: Color, t: f32) -> LinearRgba {
    let a = c1.to_linear();
    let b = c2.to_linear();

    Color::linear_rgb(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
    )
    .to_linear()
}

fn linear_to_rgba(rgb: LinearRgba) -> [f32; 4] {
    [rgb.red, rgb.green, rgb.blue, 1.0]
}

fn rgba_to_linear(rgb: LinearRgba) -> [f32; 4] {
    [rgb.red, rgb.green, rgb.blue, 1.0]
}

fn color_mix(a: LinearRgba, b: LinearRgba, t: f32) -> LinearRgba {
    Color::linear_rgb(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
    )
    .to_linear()
}

pub fn ground_plane(rng: &mut SmallRng, offset: Vec3, config: &GroundConfig) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let n_triangles = 2 * (4 as u32).pow(config.subdivisions.into());
    let n_points = n_triangles as usize * 3;

    let sub_quads = 2u32.pow(config.subdivisions.into());
    let resolution = sub_quads + 1;

    let step = TILE_SIZE / sub_quads as f32;

    let color_map = noise_map(sub_quads, &config.color, step, offset);
    let height_map = noise_map(resolution, &config.height, step, offset);

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_points);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(n_points);
    let mut indices: Vec<u32> = Vec::with_capacity(n_points);

    for z in 0..sub_quads {
        for x in 0..sub_quads {
            let half = TILE_SIZE / 2.0;
            let step = TILE_SIZE / sub_quads as f32;

            let index = (z * sub_quads + x) as usize;

            let x0 = step * x as f32 - half;
            let x1 = step * (x + 1) as f32 - half;

            let z0 = step * z as f32 - half;
            let z1 = step * (z + 1) as f32 - half;

            let base_1 = (z * resolution + x) as usize;
            let base_2 = ((z + 1) * resolution + x) as usize;

            let height = &config.height;
            let color = &config.color;
            let r_colors = &config.colors;

            // height
            let top_left = (height_map[base_1 + 0] + height.value_1) * height.value_2;
            let bot_left = (height_map[base_2 + 0] + height.value_1) * height.value_2;

            let top_right = (height_map[base_1 + 1] + height.value_1) * height.value_2;
            let bot_right = (height_map[base_2 + 1] + height.value_1) * height.value_2;

            positions.push([x0, top_left, z0]);
            positions.push([x0, bot_left, z1]);
            positions.push([x1, top_right, z0]);

            positions.push([x1, top_right, z0]);
            positions.push([x0, bot_left, z1]);
            positions.push([x1, bot_right, z1]);

            let color_index = rng.random_range(0..r_colors.len());
            let random_color = r_colors[color_index].to_linear();

            let noise_color = rgb_lerp(color.value_1, color.value_2, color_map[index]);
            let color = color_mix(random_color, noise_color, 0.5);

            colors.push(linear_to_rgba(color));
            colors.push(linear_to_rgba(color));
            colors.push(linear_to_rgba(color));

            colors.push(linear_to_rgba(color));
            colors.push(linear_to_rgba(color));
            colors.push(linear_to_rgba(color));

            indices.push((index * 6 + 0) as u32);
            indices.push((index * 6 + 1) as u32);
            indices.push((index * 6 + 2) as u32);

            indices.push((index * 6 + 3) as u32);
            indices.push((index * 6 + 4) as u32);
            indices.push((index * 6 + 5) as u32);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    mesh
}

fn noise_map<T, K>(size: u32, noise: &Noise<T, K>, step: f32, offset: Vec3) -> Vec<f32> {
    (0..size * size)
        .map(|i| {
            let x = step * (i % size) as f32 + offset.x;
            let z = step * (i / size) as f32 + offset.z;

            let mut value = 0.0;
            let mut max = 0.0;

            for octave in &noise.octaves {
                let noise = perlin_2d(x * octave.frequency, z * octave.frequency);
                value += noise * octave.amplitude;
                max += octave.amplitude;
            }

            value /= max;

            ((value + 1.0) / 2.0).clamp(0.0, 1.0)
        })
        .collect()
}
