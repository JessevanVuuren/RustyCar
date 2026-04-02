use crate::world::components::{Range, Surface, TilePos};
use std::f32::consts::FRAC_PI_2;
use std::iter;

pub const DOWN: f32 = 0.0;
pub const UP: f32 = FRAC_PI_2;
pub const LEFT: f32 = FRAC_PI_2 * 3.0;
pub const RIGHT: f32 = FRAC_PI_2 * 2.0;

pub fn range_from_surface(surface: &Surface) -> impl Iterator<Item = TilePos> {
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
