use crate::{
    Random,
    animal::{
        butterfly::components::{ButterflyPath, ButterflyState},
        components::{AnimalKind, AnimalLibrary, AnimalState, Butterfly},
    },
    world::{
        components::StaticWorld,
        tile_pos::TilePos,
        utils::{model_path, range_from_surfaces},
    },
};
use bevy::prelude::*;

pub fn spawn_butterfly(
    mut commands: Commands,
    mut random: ResMut<Random>,
    library: Res<AnimalLibrary>,
    static_world: Res<StaticWorld>,
    asset_server: Res<AssetServer>,
) {
    for roam in &static_world.animals {
        if matches!(roam.animal.kind, AnimalKind::Butterfly) {
            if let Some(animation) = library.animals.get(&roam.animal.kind) {
                let path = model_path(&mut random.rng, &roam.animal.path, &roam.animal.range);
                let range: Vec<TilePos> = range_from_surfaces(&roam.surface).collect();

                for _ in 0..roam.animal.amount {
                    commands
                        .spawn((
                            Butterfly,
                            AnimalState::Fly,
                            animation.clone(),
                            Transform::default(),
                            Visibility::default(),
                            ButterflyState::Searching,
                            GlobalTransform::default(),
                            ButterflyPath::default(),
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
}
