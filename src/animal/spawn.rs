use crate::animal::{
    animals::components::{ButterflyPath, ButterflyState},
    components::{AnimalAnimations, AnimalLibrary, AnimalState, Butterfly},
};
use bevy::prelude::*;
use std::collections::HashMap;

pub fn spawn_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let path = "models/animals/butterfly.glb";
    let mut library = AnimalLibrary::default();

    let (graph, node_animations) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(path)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(path)),
    ]);

    library.butterfly = Some(AnimalAnimations {
        graph: graphs.add(graph),
        nodes: HashMap::from([
            (AnimalState::Fly, node_animations[0]),
            (AnimalState::Idle, node_animations[1]),
        ]),
    });

    commands.insert_resource(library);
}

pub fn spawn_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    library: Res<AnimalLibrary>,
) {
    let path = "models/animals/butterfly.glb";
    let size = 0.03;
    let offset = Vec3::new(0.0, 1.2, 0.0);

    if let Some(butterfly_anim) = &library.butterfly {
        for _ in 0..1 {
            commands
                .spawn((
                    Butterfly,
                    AnimalState::Fly,
                    Transform::default(),
                    Visibility::default(),
                    butterfly_anim.clone(),
                    ButterflyState::Searching,
                    GlobalTransform::default(),
                    ButterflyPath::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
                        Transform {
                            translation: offset,
                            scale: Vec3::splat(size),
                            ..default()
                        },
                    ));
                });
        }
    }
}
