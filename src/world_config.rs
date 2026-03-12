use std::f32::consts::PI;

use bevy::prelude::*;

use crate::world::{
    components::{
        Comp, Model, Offset, Placement, Range, Rotation, StaticWorld, Surface, TilePos, TileType,
        Value, WorldBlock,
    },
    utils::{DOWN, LEFT, RIGHT, UP},
};

pub fn test_world() -> StaticWorld {
    StaticWorld {
        blocks: vec![
            //welcome to the jungle
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass,
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    placement: Placement {
                        rotation: Rotation::RandomDirection,
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-20, -20), TilePos::new(20, 20)),
                    negative: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 8),
                    comp: Comp::Flower,
                    path: "nature/flower".into(),
                    placement: Placement {
                        amount: Value::Amount(2000),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-20, -20), TilePos::new(20, 20)),
                    negative: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Log,
                    path: "nature/log".into(),
                    placement: Placement {
                        amount: Value::Amount(500),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-20, -20), TilePos::new(20, 20)),
                    negative: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Mushroom,
                    path: "nature/mushroom".into(),
                    placement: Placement {
                        amount: Value::Amount(500),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-20, -20), TilePos::new(20, 20)),
                    negative: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 6),
                    comp: Comp::Rock,
                    path: "nature/rock".into(),
                    placement: Placement {
                        amount: Value::Amount(500),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-20, -20), TilePos::new(20, 20)),
                    negative: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Tree,
                    path: "nature/pine_tree".into(),
                    placement: Placement {
                        amount: Value::Amount(250),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-20, -20), TilePos::new(20, 20)),
                    negative: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                }],
            },
            // playable world
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass,
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    placement: Placement {
                        rotation: Rotation::RandomDirection,
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 8),
                    comp: Comp::Flower,
                    path: "nature/flower".into(),
                    placement: Placement {
                        amount: Value::Amount(30),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Flower,
                    path: "nature/mushroom".into(),
                    placement: Placement {
                        amount: Value::Amount(10),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-5, -5), TilePos::new(5, 5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-4, -5), TilePos::new(4, -5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-4, 5), TilePos::new(4, 5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(UP, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(-5, -4), TilePos::new(-5, 4)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(LEFT, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(TilePos::new(5, -4), TilePos::new(5, 4)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(DOWN, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::One(TilePos::new(-5, -5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(RIGHT, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::One(TilePos::new(5, 5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(UP, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::One(TilePos::new(-5, 5)),
                    ..Default::default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(LEFT, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::One(TilePos::new(5, -5)),
                    ..Default::default()
                }],
            },
        ],
    }
}
