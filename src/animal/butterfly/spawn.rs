use crate::{
    Random,
    animal::{
        components::{
            AnimalAnimations, AnimalKind, AnimalLibrary, AnimalState, Butterfly, ButterflyBehavior,
            FlowerBed, FreeFly,
        },
        natural_fly_path::NaturalFlyPath,
        utils::animal_kind_from_static,
    },
    world::{
        components::{AnimalRoam, Flower, StaticWorld, TileWorld},
        tile_pos::TilePos,
        utils::{model_path, range_from_surfaces},
    },
};
use bevy::prelude::*;
use rand::{RngExt, rngs::SmallRng};

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
                ButterflyBehavior::Swirling => {}
                ButterflyBehavior::FlowerBed => {
                    flower_bed_behavior(&mut commands, &mut random.rng, &world, flowers, roam, id);
                }
            }
        }
    }
}

fn free_fly_behavior(commands: &mut Commands, roam: &AnimalRoam, id: Entity) {
    let range = range_from_surfaces(&roam.surface).collect();

    commands
        .entity(id)
        .insert(FreeFly(range))
        .insert(NaturalFlyPath::default())
        .insert(AnimalState::Fly);
}

fn flower_bed_behavior(
    commands: &mut Commands,
    rng: &mut SmallRng,
    world: &TileWorld,
    flowers: Query<(), With<Flower>>,
    roam: &AnimalRoam,
    id: Entity,
) {
    let flower_bed_id = rng.random_range(0..u8::MAX);
    let range: Vec<_> = range_from_surfaces(&roam.surface).collect();

    let entitys = world.all_entitys_from_range(&range);

    for entity in entitys {
        if flowers.get(entity).is_ok() {
            commands.entity(entity).insert(FlowerBed(flower_bed_id));
        }
    }

    commands
        .entity(id)
        .insert(FlowerBed(flower_bed_id))
        .insert(AnimalState::Idle);
}
