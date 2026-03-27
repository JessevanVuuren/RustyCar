use crate::{
    extra::math::{ease_in_quint, lerp, s_curve},
    world::{
        components::{Comp, Grass, Range, StaticWorld, TilePos, TileWorld},
        ground::mesh_utils::{set_mesh_colors, tile_mesh_colors, tile_mesh_positions},
        utils::range_from_surface,
    },
};
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use bevy::{mesh::VertexAttributeValues, prelude::*};
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
    let mut rng = SmallRng::seed_from_u64(1604);

    let sub_quads = 2i32.pow(4.0 as u32);
    let points = sub_quads + 1;

    for block in static_world.blocks.iter() {
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

                            mix_colors(
                                &mut rng,
                                &mut color_tl,
                                &mut color_tr,
                                &mut color_bl,
                                &mut color_br,
                            );

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
}

const PRECISION: i32 = 1000;

fn color_to_key(color: [f32; 4]) -> (i32, i32, i32) {
    (
        (color[0] * PRECISION as f32) as i32,
        (color[1] * PRECISION as f32) as i32,
        (color[2] * PRECISION as f32) as i32,
    )
}

fn key_to_color(key: (i32, i32, i32)) -> [f32; 4] {
    [
        key.0 as f32 / PRECISION as f32,
        key.1 as f32 / PRECISION as f32,
        key.2 as f32 / PRECISION as f32,
        1.0,
    ]
}

fn mix_colors(
    rng: &mut SmallRng,

    tile1: &mut [[f32; 4]],
    tile2: &mut [[f32; 4]],
    tile3: &mut [[f32; 4]],
    tile4: &mut [[f32; 4]],
) {
    let sub_quads = 2i32.pow(4.0 as u32);
    let mut colors = HashSet::new();

    for i in 0..sub_quads * 6 {
        colors.insert(color_to_key(tile1[i as usize]));
        colors.insert(color_to_key(tile2[i as usize]));
        colors.insert(color_to_key(tile3[i as usize]));
        colors.insert(color_to_key(tile4[i as usize]));
    }

    let colors: Vec<&(i32, i32, i32)> = colors.iter().collect();
    let unique = colors.len();

    for z in 0..sub_quads {
        for x in 0..sub_quads {
            let i = (z * sub_quads + x) as usize;
            let color = colors[rng.random_range(0..unique)];

            tile1[i as usize * 6 + 0] = key_to_color(*color);
            tile1[i as usize * 6 + 1] = key_to_color(*color);
            tile1[i as usize * 6 + 2] = key_to_color(*color);
            tile1[i as usize * 6 + 3] = key_to_color(*color);
            tile1[i as usize * 6 + 4] = key_to_color(*color);
            tile1[i as usize * 6 + 5] = key_to_color(*color);

            let color = colors[rng.random_range(0..unique)];

            tile2[i as usize * 6 + 0] = key_to_color(*color);
            tile2[i as usize * 6 + 1] = key_to_color(*color);
            tile2[i as usize * 6 + 2] = key_to_color(*color);
            tile2[i as usize * 6 + 3] = key_to_color(*color);
            tile2[i as usize * 6 + 4] = key_to_color(*color);
            tile2[i as usize * 6 + 5] = key_to_color(*color);

            let color = colors[rng.random_range(0..unique)];

            tile3[i as usize * 6 + 0] = key_to_color(*color);
            tile3[i as usize * 6 + 1] = key_to_color(*color);
            tile3[i as usize * 6 + 2] = key_to_color(*color);
            tile3[i as usize * 6 + 3] = key_to_color(*color);
            tile3[i as usize * 6 + 4] = key_to_color(*color);
            tile3[i as usize * 6 + 5] = key_to_color(*color);

            let color = colors[rng.random_range(0..unique)];

            tile4[i as usize * 6 + 0] = key_to_color(*color);
            tile4[i as usize * 6 + 1] = key_to_color(*color);
            tile4[i as usize * 6 + 2] = key_to_color(*color);
            tile4[i as usize * 6 + 3] = key_to_color(*color);
            tile4[i as usize * 6 + 4] = key_to_color(*color);
            tile4[i as usize * 6 + 5] = key_to_color(*color);
        }
    }
}
