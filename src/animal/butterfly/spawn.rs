use crate::{
    Random,
    animal::{
        butterfly::components::{ButterflyPath, ButterflyState},
        components::{
            AnimalAnimations, AnimalKind, AnimalLibrary, AnimalState, Butterfly, FlowerBed,
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
use rand::RngExt;

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
    let flower_bed_id = random.rng.random_range(0..u8::MAX);

    for roam in butterflies {
        let range: Vec<_> = range_from_surfaces(&roam.surface).collect();
        let entitys = world.all_entitys_from_range(&range);

        for entity in entitys {
            if flowers.get(entity).is_ok() {
                commands.entity(entity).insert(FlowerBed(flower_bed_id));
            }
        }

        for _ in 0..roam.animal.amount {
            let path = model_path(&mut random.rng, &roam.animal.path, &roam.animal.range);
            let animation = library.animals.get(&path).unwrap();
            
            let mut offset = roam.animal.offset.clone();
            let variation = -roam.animal.variation..roam.animal.variation;
            offset.scale += Vec3::splat(random.rng.random_range(variation));


            commands
                .spawn((
                    Butterfly,
                    AnimalState::Fly,
                    animation.clone(),
                    Transform::default(),
                    Visibility::default(),
                    FlowerBed(flower_bed_id),
                    ButterflyPath::default(),
                    ButterflyState::Searching,
                    GlobalTransform::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SceneRoot(
                            asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone())),
                        ),
                        offset,
                    ));
                });
        }
    }
}
