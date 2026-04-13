use crate::world::{components::TileWorld, tile_pos::TilePos};
use bevy::prelude::*;

pub fn world_entitys_from_range(
    range: &[TilePos],
    world: &TileWorld,
) -> impl Iterator<Item = Entity> {
    range
        .iter()
        .flat_map(|tile| world.models.get(tile).into_iter().flatten().copied())
}
