use crate::{
    Random,
    animal::{
        butterfly::components::{ButterflyPath, ButterflyState},
        components::{AnimalKind, AnimalLibrary, AnimalState, Butterfly, FlowerBed},
        utils::world_entitys_from_range,
    },
    world::{
        components::{Flower, StaticWorld, TileWorld},
        tile_pos::TilePos,
        utils::{model_path, range_from_surfaces},
    },
};
use bevy::prelude::*;

pub fn spawn_butterfly(
    mut commands: Commands,
    mut random: ResMut<Random>,
    library: Res<AnimalLibrary>,
    mut world: ResMut<TileWorld>,
    static_world: Res<StaticWorld>,
    asset_server: Res<AssetServer>,
) {
    for roam in &static_world.animals {
        if let Some(animation) = library.animals.get(&roam.animal.kind)
            && matches!(roam.animal.kind, AnimalKind::Butterfly)
        {
            let path = model_path(&mut random.rng, &roam.animal.path, &roam.animal.range);
            let range: Vec<_> = range_from_surfaces(&roam.surface).collect();
            let entitys = world_entitys_from_range(&range, &world);

            for entity in entitys {
                commands.entity(entity).insert(FlowerBed(1));
            }

            for _ in 0..roam.animal.amount {
                commands
                    .spawn((
                        Butterfly,
                        FlowerBed(1),
                        AnimalState::Fly,
                        animation.clone(),
                        Transform::default(),
                        Visibility::default(),
                        ButterflyPath::default(),
                        ButterflyState::Searching,
                        GlobalTransform::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SceneRoot(
                                asset_server
                                    .load(GltfAssetLabel::Scene(0).from_asset(path.clone())),
                            ),
                            roam.animal.offset.clone(),
                        ));
                    });
            }
        }
    }
}
