use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use rand::{RngExt, SeedableRng, rngs::SmallRng};

use crate::{extra::noise::perlin_2d, world::components::GrassConfig};

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

pub fn grass_plane(offset: Vec3, subdivision: u32, size: f32, config: GrassConfig) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let n_triangles = 2 * (4 as u32).pow(subdivision);
    let n_points = n_triangles as usize * 3;

    let sub_quads = 2u32.pow(subdivision);
    let resolution = sub_quads + 1;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_points);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(n_points);
    let mut indices: Vec<u32> = Vec::with_capacity(n_points);

    let mut rng = SmallRng::seed_from_u64(1604);

    let height_map: Vec<_> = (0..2u32.pow(resolution))
        .map(|_| rng.random_range(0.0..0.5))
        .collect();

    let mut i = 0;
    for z in 0..sub_quads {
        for x in 0..sub_quads {
            let step = size / sub_quads as f32;
            let half = size / 2.0;

            let x0 = step * x as f32 - half;
            let x1 = step * (x + 1) as f32 - half;

            let z0 = step * z as f32 - half;
            let z1 = step * (z + 1) as f32 - half;

            let base_1 = (z * resolution + x) as usize;
            let base_2 = ((z + 1) * resolution + x) as usize;

            positions.push([x0, height_map[base_1 + 0], z0]);
            positions.push([x0, height_map[base_2 + 0], z1]);
            positions.push([x1, height_map[base_1 + 1], z0]);

            positions.push([x1, height_map[base_1 + 1], z0]);
            positions.push([x0, height_map[base_2 + 0], z1]);
            positions.push([x1, height_map[base_2 + 1], z1]);

            // let color_index = rng.random_range(0..config.colors.len());
            // let color_scale = config.colors[color_index].to_linear();

            // colors.push([color_scale.red, color_scale.green, color_scale.blue, 1.0]);
            // colors.push([color_scale.red, color_scale.green, color_scale.blue, 1.0]);
            // colors.push([color_scale.red, color_scale.green, color_scale.blue, 1.0]);

            // let color_index = rng.random_range(0..config.colors.len());
            // let color_scale = config.colors[color_index].to_linear();

            colors.push([1.0, 1.0, 1.0, 1.0]);
            colors.push([1.0, 1.0, 1.0, 1.0]);
            colors.push([1.0, 1.0, 1.0, 1.0]);

            colors.push([0.0, 0.0, 0.0, 1.0]);
            colors.push([0.0, 0.0, 0.0, 1.0]);
            colors.push([0.0, 0.0, 0.0, 1.0]);

            indices.push(i + 0);
            indices.push(i + 1);
            indices.push(i + 2);

            indices.push(i + 3);
            indices.push(i + 4);
            indices.push(i + 5);
            i += 6;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    mesh.compute_normals();

    mesh
}
