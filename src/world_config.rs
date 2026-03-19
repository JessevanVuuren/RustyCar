use std::f32::consts::PI;

use bevy::prelude::*;

use crate::world::{
    components::{
        Comp, GrassConfig, Model, Noise, NoiseLevel, Offset, Placement, Range, Rotation,
        StaticWorld, Surface, TilePos, TileType, Value, WorldBlock,
    },
    utils::{DOWN, LEFT, RIGHT, UP},
};

pub fn grass_test_2() -> StaticWorld {
    let dirt_start = TilePos::new(2, 2);
    let dirt_stop = TilePos::new(3, 3);
    let grass_start = TilePos::new(1, 1);
    let grass_stop = TilePos::new(4, 4);

    StaticWorld {
        blocks: vec![
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            value_2: Color::linear_rgb(0.275, 0.412, 0.0),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.8,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 1.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.35, 0.18, 0.05),
                            Color::linear_rgb(0.5, 0.31, 0.14),
                            Color::linear_rgb(0.58, 0.4, 0.22),
                            Color::linear_rgb(0.65, 0.54, 0.39),
                            Color::linear_rgb(0.71, 0.68, 0.56),
                            Color::linear_rgb(0.76, 0.77, 0.67),
                            Color::linear_rgb(0.64, 0.67, 0.53),
                            Color::linear_rgb(0.4, 0.43, 0.29),
                            Color::linear_rgb(0.25, 0.28, 0.2),
                            Color::linear_rgb(0.2, 0.24, 0.16),
                        ],
                        subdivisions: 4,
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(dirt_start, dirt_stop),
                    ..default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.01,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.0, 0.69, 0.22),
                            value_2: Color::linear_rgb(0.624, 1.0, 0.745),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.2,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.5,
                        },
                        subdivisions: 4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(grass_start, grass_stop),
                    negative: Range::Range(dirt_start, dirt_stop),
                }],
            },
        ],
    }
}

pub fn grass_test() -> StaticWorld {
    let dirt_start = TilePos::new(3, 3);
    let dirt_stop = TilePos::new(8, 8);
    let grass_start = TilePos::new(1, 1);
    let grass_stop = TilePos::new(10, 10);

    let dirt_start2 = TilePos::new(13, 2);
    let dirt_stop2 = TilePos::new(13, 2);
    let grass_start2 = TilePos::new(12, 1);
    let grass_stop2 = TilePos::new(14, 3);

    let dirt_start3 = TilePos::new(17, 2);
    let dirt_stop3 = TilePos::new(21, 2);
    let grass_start3 = TilePos::new(16, 1);
    let grass_stop3 = TilePos::new(22, 3);

    StaticWorld {
        blocks: vec![
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            value_2: Color::linear_rgb(0.275, 0.412, 0.0),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.8,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 1.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.35, 0.18, 0.05),
                            Color::linear_rgb(0.5, 0.31, 0.14),
                            Color::linear_rgb(0.58, 0.4, 0.22),
                            Color::linear_rgb(0.65, 0.54, 0.39),
                            Color::linear_rgb(0.71, 0.68, 0.56),
                            Color::linear_rgb(0.76, 0.77, 0.67),
                            Color::linear_rgb(0.64, 0.67, 0.53),
                            Color::linear_rgb(0.4, 0.43, 0.29),
                            Color::linear_rgb(0.25, 0.28, 0.2),
                            Color::linear_rgb(0.2, 0.24, 0.16),
                        ],
                        subdivisions: 4,
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(dirt_start, dirt_stop),
                    ..default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.01,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.0, 0.69, 0.22),
                            value_2: Color::linear_rgb(0.624, 1.0, 0.745),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.2,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.5,
                        },
                        subdivisions: 4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(grass_start, grass_stop),
                    negative: Range::Range(dirt_start, dirt_stop),
                }],
            },
            //
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            value_2: Color::linear_rgb(0.275, 0.412, 0.0),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.8,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 1.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.35, 0.18, 0.05),
                            Color::linear_rgb(0.5, 0.31, 0.14),
                            Color::linear_rgb(0.58, 0.4, 0.22),
                            Color::linear_rgb(0.65, 0.54, 0.39),
                            Color::linear_rgb(0.71, 0.68, 0.56),
                            Color::linear_rgb(0.76, 0.77, 0.67),
                            Color::linear_rgb(0.64, 0.67, 0.53),
                            Color::linear_rgb(0.4, 0.43, 0.29),
                            Color::linear_rgb(0.25, 0.28, 0.2),
                            Color::linear_rgb(0.2, 0.24, 0.16),
                        ],
                        subdivisions: 4,
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(dirt_start2, dirt_stop2),
                    ..default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.01,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.0, 0.69, 0.22),
                            value_2: Color::linear_rgb(0.624, 1.0, 0.745),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.2,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.5,
                        },
                        subdivisions: 4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(grass_start2, grass_stop2),
                    negative: Range::Range(dirt_start2, dirt_stop2),
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            value_2: Color::linear_rgb(0.275, 0.412, 0.0),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.8,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 1.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.35, 0.18, 0.05),
                            Color::linear_rgb(0.5, 0.31, 0.14),
                            Color::linear_rgb(0.58, 0.4, 0.22),
                            Color::linear_rgb(0.65, 0.54, 0.39),
                            Color::linear_rgb(0.71, 0.68, 0.56),
                            Color::linear_rgb(0.76, 0.77, 0.67),
                            Color::linear_rgb(0.64, 0.67, 0.53),
                            Color::linear_rgb(0.4, 0.43, 0.29),
                            Color::linear_rgb(0.25, 0.28, 0.2),
                            Color::linear_rgb(0.2, 0.24, 0.16),
                        ],
                        subdivisions: 4,
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(dirt_start3, dirt_stop3),
                    ..default()
                }],
            },
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.01,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.0, 0.69, 0.22),
                            value_2: Color::linear_rgb(0.624, 1.0, 0.745),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.2,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.5,
                        },
                        subdivisions: 4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(grass_start3, grass_stop3),
                    negative: Range::Range(dirt_start3, dirt_stop3),
                }],
            },
        ],
    }
}

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
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            value_2: Color::linear_rgb(0.275, 0.412, 0.0),
                        },
                        subdivisions: 4,
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.8,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.5,
                        },

                        colors: vec![
                            Color::linear_rgb(0.35, 0.18, 0.05),
                            Color::linear_rgb(0.5, 0.31, 0.14),
                            Color::linear_rgb(0.58, 0.4, 0.22),
                            Color::linear_rgb(0.65, 0.54, 0.39),
                            Color::linear_rgb(0.71, 0.68, 0.56),
                            Color::linear_rgb(0.76, 0.77, 0.67),
                            Color::linear_rgb(0.64, 0.67, 0.53),
                            Color::linear_rgb(0.4, 0.43, 0.29),
                            Color::linear_rgb(0.25, 0.28, 0.2),
                            Color::linear_rgb(0.2, 0.24, 0.16),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(jungle_start, play_stop),
                    negative: Range::Range(play_start, play_stop),
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
            WorldBlock {
                objects: vec![Model {
                    range: Range::None,
                    comp: Comp::Grass(GrassConfig {
                        subdivisions: 4,
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.01,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.0, 0.69, 0.22),
                            value_2: Color::linear_rgb(0.624, 1.0, 0.745),
                        },
                        height: Noise {
                            octaves: vec![
                                NoiseLevel {
                                    frequency: 0.1,
                                    amplitude: 1.0,
                                },
                                NoiseLevel {
                                    frequency: 5.6,
                                    amplitude: 0.2,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.5,
                        },

                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    tile_type: TileType::Ground,
                    ..Default::default()
                }],
                surface: vec![Surface {
                    positive: Range::Range(play_start, play_stop),
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
