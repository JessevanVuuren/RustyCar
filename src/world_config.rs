use std::f32::consts::PI;

use bevy::prelude::*;

use crate::world::{
    components::{
        Comp, LandConfig, Model, Noise, NoiseLevel, Offset, Placement, Range, Rotation,
        StaticWorld, Surface, TilePos, TileType, Value, WorldBlock,
    },
    utils::{DOWN, LEFT, RIGHT, UP},
};

pub fn multiple_surface() -> StaticWorld {
    StaticWorld {
        blocks: vec![WorldBlock {
            models: TileType::Ground(Model {
                comp: Comp::Land(LandConfig {
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
                    color_samples: 100,
                    color_spread: 0.4,
                    stitch_intensity: 2.0,
                    stitch_spread: 0.4,
                }),
                ..Default::default()
            }),
            surface: Surface {
                positive: vec![
                    Range::Range(TilePos::new(1, 1), TilePos::new(1, 4)),
                    Range::Range(TilePos::new(3, 1), TilePos::new(3, 4)),
                ],
                negative: vec![
                    Range::One(TilePos::new(1, 2)),
                    Range::One(TilePos::new(3, 3)),
                ],
            },
        }],
    }
}

pub fn small_grass() -> StaticWorld {
    // let dirt_start = TilePos::new(3, 3);
    // let dirt_stop = TilePos::new(6, 6);
    // let grass_start = TilePos::new(1, 1);
    // let grass_stop = TilePos::new(8, 8);
    let dirt_start = TilePos::new(1, 1);
    let dirt_stop = TilePos::new(2, 1);
    let grass_start = TilePos::new(1, 2);
    let grass_stop = TilePos::new(2, 2);

    StaticWorld {
        blocks: vec![
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            // value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            // value_2: Color::linear_rgb(0.275, 0.412, 0.0),
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(dirt_start, dirt_stop)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(grass_start, grass_stop)],
                    negative: vec![Range::Range(dirt_start, dirt_stop)],
                },
            },
        ],
    }
}

pub fn lots_of_patches() -> StaticWorld {
    let dirt_start_patch = TilePos::new(1, 1);
    let dirt_stop_patch = TilePos::new(2, 1);

    let grass_start_patch = TilePos::new(1, 1);
    let grass_stop_patch = TilePos::new(3, 4);

    let grass_start_neg = TilePos::new(1, 1);
    let grass_stop_neg = TilePos::new(2, 3);

    let gravel_start = TilePos::new(1, 2);
    let gravel_stop = TilePos::new(2, 3);

    let spooky = TilePos::new(5, 1);
    let gravel2 = TilePos::new(6, 1);
    let dirt2 = TilePos::new(5, 2);
    let grass2 = TilePos::new(6, 2);

    StaticWorld {
        blocks: vec![
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    path: "ground/grass".into(),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(dirt_start_patch, dirt_stop_patch)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    path: "ground/grass".into(),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(grass_start_patch, grass_stop_patch)],
                    negative: vec![Range::Range(grass_start_neg, grass_stop_neg)],
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                                    frequency: 3.6,
                                    amplitude: 0.6,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 2.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.729, 0.729, 0.729),
                            Color::linear_rgb(0.471, 0.471, 0.471),
                            Color::linear_rgb(0.8, 0.8, 0.8),
                            Color::linear_rgb(0.212, 0.212, 0.212),
                        ],
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(gravel_start, gravel_stop)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(dirt2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(grass2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                                    frequency: 3.6,
                                    amplitude: 0.6,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 2.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.729, 0.729, 0.729),
                            Color::linear_rgb(0.471, 0.471, 0.471),
                            Color::linear_rgb(0.8, 0.8, 0.8),
                            Color::linear_rgb(0.212, 0.212, 0.212),
                        ],
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(gravel2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                                    frequency: 2.6,
                                    amplitude: 0.9,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.3,
                        },
                        colors: vec![
                            Color::linear_rgb(0.824, 0.0, 1.0),
                            Color::linear_rgb(0.729, 0.212, 0.839),
                            Color::linear_rgb(0.51, 0.039, 0.612),
                            Color::linear_rgb(0.969, 0.831, 1.0),
                        ],
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(spooky)],
                    ..default()
                },
            },
        ],
    }
}

pub fn large_grass_test() -> StaticWorld {
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

    let dirt_start_patch = TilePos::new((1 + 11), (1 + 4));
    let dirt_stop_patch = TilePos::new((2 + 11), (1 + 4));

    let grass_start_patch = TilePos::new((1 + 11), (1 + 4));
    let grass_stop_patch = TilePos::new((3 + 11), (4 + 4));

    let grass_start_neg = TilePos::new((1 + 11), (1 + 4));
    let grass_stop_neg = TilePos::new((2 + 11), (3 + 4));

    let gravel_start = TilePos::new((1 + 11), (2 + 4));
    let gravel_stop = TilePos::new((2 + 11), (3 + 4));

    let spooky = TilePos::new((5 + 11), (1 + 4));
    let gravel2 = TilePos::new((6 + 11), (1 + 4));
    let dirt2 = TilePos::new((5 + 11), (2 + 4));
    let grass2 = TilePos::new((6 + 11), (2 + 4));

    StaticWorld {
        blocks: vec![
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(dirt_start, dirt_stop)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(grass_start, grass_stop)],
                    negative: vec![Range::Range(dirt_start, dirt_stop)],
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(dirt_start2, dirt_stop2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(grass_start2, grass_stop2)],
                    negative: vec![Range::Range(dirt_start2, dirt_stop2)],
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(dirt_start3, dirt_stop3)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(grass_start3, grass_stop3)],
                    negative: vec![Range::Range(dirt_start3, dirt_stop3)],
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(dirt_start_patch, dirt_stop_patch)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(grass_start_patch, grass_stop_patch)],
                    negative: vec![Range::Range(grass_start_neg, grass_stop_neg)],
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                                    frequency: 3.6,
                                    amplitude: 0.6,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 2.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.729, 0.729, 0.729),
                            Color::linear_rgb(0.471, 0.471, 0.471),
                            Color::linear_rgb(0.8, 0.8, 0.8),
                            Color::linear_rgb(0.212, 0.212, 0.212),
                        ],
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(gravel_start, gravel_stop)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(dirt2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                        colors: vec![
                            Color::linear_rgb(0.125, 0.545, 0.227),
                            Color::linear_rgb(0.145, 0.635, 0.267),
                            Color::linear_rgb(0.176, 0.776, 0.325),
                            Color::linear_rgb(0.29, 0.839, 0.427),
                        ],
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(grass2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                                    frequency: 3.6,
                                    amplitude: 0.6,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 2.0,
                        },
                        colors: vec![
                            Color::linear_rgb(0.729, 0.729, 0.729),
                            Color::linear_rgb(0.471, 0.471, 0.471),
                            Color::linear_rgb(0.8, 0.8, 0.8),
                            Color::linear_rgb(0.212, 0.212, 0.212),
                        ],
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(gravel2)],
                    ..default()
                },
            },
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
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
                                    frequency: 2.6,
                                    amplitude: 0.9,
                                },
                            ],
                            value_1: 0.0,
                            value_2: 0.3,
                        },
                        colors: vec![
                            Color::linear_rgb(0.824, 0.0, 1.0),
                            Color::linear_rgb(0.729, 0.212, 0.839),
                            Color::linear_rgb(0.51, 0.039, 0.612),
                            Color::linear_rgb(0.969, 0.831, 1.0),
                        ],
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
                    }),
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::One(spooky)],
                    ..default()
                },
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
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
                        color: Noise {
                            octaves: vec![NoiseLevel {
                                frequency: 0.1,
                                amplitude: 1.0,
                            }],
                            value_1: Color::linear_rgb(0.678, 0.369, 0.012),
                            value_2: Color::linear_rgb(0.275, 0.412, 0.0),
                        },
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
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
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(jungle_start, play_stop)],
                    negative: vec![Range::Range(play_start, play_stop)],
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),
                surface: Surface {
                    positive: vec![Range::Range(jungle_start, jungle_stop)],
                    negative: vec![Range::Range(play_start, jungle_stop)],
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),
                surface: Surface {
                    positive: vec![Range::Range(jungle_start, jungle_stop)],
                    negative: vec![Range::Range(play_start, jungle_stop)],
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),

                surface: Surface {
                    positive: vec![Range::Range(jungle_start, jungle_stop)],
                    negative: vec![Range::Range(play_start, jungle_stop)],
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),

                surface: Surface {
                    positive: vec![Range::Range(jungle_start, jungle_stop)],
                    negative: vec![Range::Range(play_start, jungle_stop)],
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),

                surface: Surface {
                    positive: vec![Range::Range(jungle_start, jungle_stop)],
                    negative: vec![Range::Range(play_start, jungle_stop)],
                },
            },
            // playable world
            WorldBlock {
                models: TileType::Ground(Model {
                    range: Range::None,
                    comp: Comp::Land(LandConfig {
                        subdivisions: 4,
                        color_samples: 100,
                        color_spread: 0.4,
                        stitch_intensity: 2.0,
                        stitch_spread: 0.4,
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
                    ..Default::default()
                }),
                surface: Surface {
                    positive: vec![Range::Range(play_start, play_stop)],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),
                surface: Surface {
                    positive: vec![Range::Range(play_start, play_stop)],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
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
                }]),
                surface: Surface {
                    positive: vec![Range::Range(play_start, play_stop)],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::Range(
                        TilePos::new(play_start.x + 1, play_start.z),
                        TilePos::new(play_stop.x - 1, play_start.z),
                    )],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::Range(
                        TilePos::new(play_start.x + 1, play_stop.z),
                        TilePos::new(play_stop.z - 1, play_stop.z),
                    )],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(UP, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::Range(
                        TilePos::new(play_start.x, play_start.z + 1),
                        TilePos::new(play_start.x, play_stop.z - 1),
                    )],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 4),
                    comp: Comp::Fence,
                    path: "infra/fence".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(LEFT, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::Range(
                        TilePos::new(play_stop.x, play_start.z + 1),
                        TilePos::new(play_stop.x, play_stop.z - 1),
                    )],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(DOWN, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::One(play_start)],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(RIGHT, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::One(jungle_stop)],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(UP, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::One(TilePos::new(play_start.x, play_stop.z))],
                    ..Default::default()
                },
            },
            WorldBlock {
                models: TileType::Models(vec![Model {
                    range: Range::Range(1, 2),
                    comp: Comp::Fence,
                    path: "infra/fence_corner".into(),
                    placement: Placement {
                        rotation: Rotation::Amount(LEFT, Dir3::Y),
                        ..Default::default()
                    },
                    ..Default::default()
                }]),
                surface: Surface {
                    positive: vec![Range::One(TilePos::new(play_stop.x, play_start.z))],
                    ..Default::default()
                },
            },
        ],
    }
}
