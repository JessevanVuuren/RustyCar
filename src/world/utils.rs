use crate::world::{
    components::{
        BASE_ASSET, Comp, Dirt, Fence, Flower, GroundId, Ground, GroundConfig, Log, Model, Mushroom,
        Object, Offset, Placement, Range, Rock, Rotation, StaticWorld, Surface, TileType,
        TileWorld, Tree, Value,
    },
    tile_pos::TilePos,
};
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use std::f32::consts::FRAC_PI_2;
use std::iter;

pub fn range_from_surfaces(surface: &Surface) -> impl Iterator<Item = TilePos> {
    let positive = surface.positive.iter().flat_map(|positive| match positive {
        Range::None => panic!("Surface range cant be None"),
        Range::Range(start, stop) => start.row_major(*stop),
        Range::One(place) => place.row_major(*place),
    });

    let negative = surface.negative.iter().flat_map(|negative| {
        let iter: Vec<TilePos> = match negative {
            Range::None => vec![],
            Range::Range(start, stop) => start.row_major(*stop).collect(),
            Range::One(place) => place.row_major(*place).collect(),
        };
        iter.into_iter()
    });

    TilePos::subtract_range(positive, negative)
}

pub fn add_component_to_entity(commands: &mut Commands, component: &Comp, id: Entity) {
    match component {
        Comp::Mushroom => commands.entity(id).insert(Mushroom),
        Comp::Flower => commands.entity(id).insert(Flower),
        Comp::Fence => commands.entity(id).insert(Fence),
        Comp::Tree => commands.entity(id).insert(Tree),
        Comp::Rock => commands.entity(id).insert(Rock),
        Comp::Dirt => commands.entity(id).insert(Dirt),
        Comp::Log => commands.entity(id).insert(Log),
        Comp::None => panic!("None is invalid Component type"),
        Comp::Ground(_) => panic!("Non configurable land Tile"),
    };
}

pub fn apply_transformations(rng: &mut SmallRng, transform: &mut Transform, placement: &Placement) {
    apply_rotation(rng, transform, placement);
    apply_offset(rng, transform, placement);
    apply_scale(rng, transform, placement);
}

pub fn apply_rotation(rng: &mut SmallRng, transform: &mut Transform, placement: &Placement) {
    match placement.rotation {
        Rotation::Amount(angle, axis) => transform.rotate_axis(axis, angle),
        Rotation::Random(a, b) => {
            let angle = rng.random_range(a..b);
            transform.rotation *= Quat::from_rotation_y(angle);
        }
        Rotation::RandomDirection => {
            let angle = rng.random_range(0..=3);
            transform.rotation *= Quat::from_rotation_y(angle as f32 * FRAC_PI_2);
        }
        Rotation::True => {}
    }
}

pub fn apply_offset(rng: &mut SmallRng, transform: &mut Transform, placement: &Placement) {
    match placement.offset {
        Offset::Fixed(vec) => transform.translation += vec,
        Offset::RandomInTile => transform.translation += TilePos::random_tile_offset(rng),
        Offset::RandomRange(vec3) => {
            let x = rng.random::<f32>() * vec3.x;
            let y = rng.random::<f32>() * vec3.y;
            let z = rng.random::<f32>() * vec3.z;
            transform.translation += Vec3::new(x, y, z)
        }
        Offset::Zero => (),
    }
}

pub fn apply_scale(rng: &mut SmallRng, transform: &mut Transform, placement: &Placement) {
    match placement.scale {
        Value::Amount(amount) => transform.scale *= amount,
        Value::Random(a, b) => transform.scale *= rng.random_range(a..b),
        Value::True => (),
    }
}

pub fn spawn_object(
    transform: &Transform,
    path: &str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            Object,
            transform.clone(),
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.to_string()))),
        ))
        .id()
}

pub fn model_path(rng: &mut SmallRng, model: &Model) -> String {
    let path = format!("{BASE_ASSET}{}", model.path);

    match model.range {
        Range::None => format!("{path}.glb"),
        Range::One(i) => format!("{path}_{i}.glb"),
        Range::Range(a, b) => format!("{path}_{}.glb", rng.random_range(a..=b)),
    }
}

pub fn tiles_range_from_placement(
    rng: &mut SmallRng,
    value: &Value<i32>,
    range: Vec<TilePos>,
) -> Vec<TilePos> {
    match value {
        Value::Random(_, _) => todo!("Random not implemented"),
        Value::True => range,
        Value::Amount(i) => (0..*i)
            .map(|_| range[rng.random_range(0..range.len())])
            .collect(),
    }
}
