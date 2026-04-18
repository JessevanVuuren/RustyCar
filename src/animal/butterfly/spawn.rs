use crate::{
    Random,
    animal::{
        behaviors::{NaturalFlyPath, SwirlingPath},
        components::{
            AnimalAnimations, AnimalKind, AnimalLibrary, AnimalState, Butterfly, ButterflyBehavior,
            FlowerBed, FreeFly, Swirling,
        },
        utils::animal_kind_from_static,
    },
    world::{
        components::{AnimalRoam, Flower, StaticWorld, TileWorld},
        tile_pos::TilePos,
        utils::{model_path, range_from_surfaces},
    },
};
use bevy::prelude::*;
use rand::{RngExt, rngs::SmallRng, seq::IndexedRandom};

pub fn spawn_butterfly(
    mut commands: Commands,
    mut random: ResMut<Random>,
    library: Res<AnimalLibrary>,
    mut world: ResMut<TileWorld>,
    static_world: Res<StaticWorld>,
    asset_server: Res<AssetServer>,
    flowers: Query<(), With<Flower>>,
) {
    let butterflies = animal_kind_from_static(&static_world, AnimalKind::Butterfly);

    for roam in butterflies {
        let range: Vec<_> = range_from_surfaces(&roam.surface).collect();
        let tile = range.choose(&mut random.rng).unwrap_or(&TilePos::ZERO);
        let flower_bed_id = random.rng.random_range(0..u8::MAX);

        for _ in 0..roam.animal.amount {
            let path = model_path(&mut random.rng, &roam.animal.path, &roam.animal.range);
            let animation = library.animals.get(&path).unwrap();

            let mut offset = roam.animal.offset.clone();
            let variation = -roam.animal.variation..roam.animal.variation;
            offset.scale += Vec3::splat(random.rng.random_range(variation));

            let id = commands
                .spawn((
                    Butterfly,
                    animation.clone(),
                    Transform::default(),
                    Visibility::default(),
                    GlobalTransform::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SceneRoot(
                            asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone())),
                        ),
                        offset,
                    ));
                })
                .id();

            match roam.animal.behavior {
                ButterflyBehavior::FreeFly => {
                    free_fly_behavior(&mut commands, roam, id);
                }
                ButterflyBehavior::Swirling => {
                    swirling_behavior(&mut commands, &mut random.rng, roam, id, *tile);
                }
                ButterflyBehavior::FlowerBed => {
                    flower_bed_behavior(&mut commands, &world, flowers, roam, flower_bed_id, id);
                }
            }
        }
    }
}

fn swirling_behavior(
    commands: &mut Commands,
    rng: &mut SmallRng,
    roam: &AnimalRoam,
    id: Entity,
    tile: TilePos,
) {
    let point = tile.to_world_transform();
    let path = SwirlingPath::random(rng, point.translation);

    commands
        .entity(id)
        .insert(path)
        .insert(Swirling)
        .insert(AnimalState::Fly);
}

fn free_fly_behavior(commands: &mut Commands, roam: &AnimalRoam, id: Entity) {
    let range = range_from_surfaces(&roam.surface).collect();

    commands
        .entity(id)
        .insert(FreeFly(range))
        .insert(AnimalState::Idle);
}

fn flower_bed_behavior(
    commands: &mut Commands,
    world: &TileWorld,
    flowers: Query<(), With<Flower>>,
    roam: &AnimalRoam,
    flower_id: u8,
    id: Entity,
) {
    let range: Vec<_> = range_from_surfaces(&roam.surface).collect();

    let entitys = world.all_entitys_from_range(&range);

    for entity in entitys {
        if flowers.get(entity).is_ok() {
            commands.entity(entity).insert(FlowerBed(flower_id));
        }
    }

    commands
        .entity(id)
        .insert(FlowerBed(flower_id))
        .insert(AnimalState::Idle);
}
