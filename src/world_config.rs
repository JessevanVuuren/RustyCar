use std::f32::consts::PI;

use bevy::prelude::*;

use crate::world::{
    components::{
        Comp, GrassConfig, Model, Noise, Offset, Placement, Range, Rotation, StaticWorld, Surface,
        TilePos, TileType, Value, WorldBlock,
    },
    utils::{DOWN, LEFT, RIGHT, UP},
};


pub fn test_world() -> StaticWorld {
    let jungle_start = TilePos::new(1, 1);
    let jungle_stop = TilePos::new(20, 20);

    let play_start = TilePos::new(10, 10);
    let play_stop = TilePos::new(20, 20);

    let scaler = 3;
    let flowers = 2000 / scaler;
    let log = 500 / scaler;
    let mushroom = 500 / scaler;
    let rock = 500 / scaler;
    let pine_tree = 250 / scaler;

    StaticWorld {
        blocks: vec![
            //welcome to the jungle
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: {
                            Noise {
                                scale: 40.0,
                                val1: Color::linear_rgb(0.0, 0.69, 0.22),
                                val2: Color::linear_rgb(0.624, 1.0, 0.745),
                            }
                        },
                        height: {
                            Noise {
                                scale: 0.1,
                                val1: 0.0,
                                val2: 0.05,
                            }
                        },
                        colors: vec![
                            Color::linear_rgb(1.0, 1.0, 1.0),
                            Color::linear_rgb(0.0, 0.0, 0.0),
                            // Color::linear_rgb(0.125, 0.545, 0.227),
                            // Color::linear_rgb(0.145, 0.635, 0.267),
                            // Color::linear_rgb(0.176, 0.776, 0.325),
                            // Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, play_stop),
                    ..default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 8),
                    comp: Comp::Flower,
                    path: "nature/flower".into(),
                    placement: Placement {
                        amount: Value::Amount(flowers),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, jungle_stop),
                    negative: Range::Range(play_start, jungle_stop),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Log,
                    path: "nature/log".into(),
                    placement: Placement {
                        amount: Value::Amount(log),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, jungle_stop),
                    negative: Range::Range(play_start, jungle_stop),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Mushroom,
                    path: "nature/mushroom".into(),
                    placement: Placement {
                        amount: Value::Amount(mushroom),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, jungle_stop),
                    negative: Range::Range(play_start, jungle_stop),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 6),
                    comp: Comp::Rock,
                    path: "nature/rock".into(),
                    placement: Placement {
                        amount: Value::Amount(rock),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, jungle_stop),
                    negative: Range::Range(play_start, jungle_stop),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Tree,
                    path: "nature/pine_tree".into(),
                    placement: Placement {
                        amount: Value::Amount(pine_tree),
                        offset: Offset::Random,
                        scale: Value::Random(0.2, 1.0),
                        rotation: Rotation::Random(-PI, PI),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, jungle_stop),
                    negative: Range::Range(play_start, jungle_stop),
                }],
            },
            // playable world
            // WorldBlock {
            //     objects: vec![Model {
            //         range: Range::None,
            //         comp: Comp::Grass(
            //             Noise {
            //                 scale: 1.1,
            //                 val1: Color::linear_rgb(0.0, 0.69, 0.22),
            //                 val2: Color::linear_rgb(0.624, 1.0, 0.745),
            //             },
            //             Noise {
            //                 scale: 1.0,
            //                 val1: 0.0,
            //                 val2: 0.0,
            //             },
            //         ),
            //         path: "ground/grass".into(),
            //         tile_type: TileType::Ground,
            //         ..Default::default()
            //     }],
            //     surface: vec![Surface {
            //         positive: Range::Range(play_start, play_stop),
            //         ..Default::default()
            //     }],
            // },
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
                    positive: Range::Range(play_start, play_stop),
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
                    positive: Range::Range(play_start, play_stop),
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
                    positive: Range::Range(
                        TilePos::new(play_start.x + 1, play_start.z),
                        TilePos::new(play_stop.x - 1, play_start.z),
                    ),
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
                    positive: Range::Range(
                        TilePos::new(play_start.x + 1, play_stop.z),
                        TilePos::new(play_stop.z - 1, play_stop.z),
                    ),
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
                    positive: Range::Range(
                        TilePos::new(play_start.x, play_start.z + 1),
                        TilePos::new(play_start.x, play_stop.z - 1),
                    ),
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
                    positive: Range::Range(
                        TilePos::new(play_stop.x, play_start.z + 1),
                        TilePos::new(play_stop.x, play_stop.z - 1),
                    ),
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
                    positive: Range::One(play_start),
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
                    positive: Range::One(jungle_stop),
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
                    positive: Range::One(TilePos::new(play_start.x, play_stop.z)),
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
                    positive: Range::One(TilePos::new(play_stop.x, play_start.z)),
                    ..Default::default()
                }],
            },
        ],
    }
}
