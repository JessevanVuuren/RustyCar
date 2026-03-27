use crate::world::components::{Range, Surface, TilePos};
use std::f32::consts::FRAC_PI_2;
use std::iter;

pub const DOWN: f32 = 0.0;
pub const UP: f32 = FRAC_PI_2;
pub const LEFT: f32 = FRAC_PI_2 * 3.0;
pub const RIGHT: f32 = FRAC_PI_2 * 2.0;

pub fn range_from_surface(surface: &Surface) -> impl Iterator<Item = TilePos> {
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

    TilePos::subtract_range(positive, negative)
}
