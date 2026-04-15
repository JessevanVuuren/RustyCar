use std::collections::HashMap;

use crate::{
    Random,
    animal::components::{AnimalAnimations, AnimalKind, AnimalLibrary},
    extra::components::Range,
    world::{components::StaticWorld, utils::every_model_path},
};
use bevy::prelude::*;

pub fn spawn_animations(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut library = AnimalLibrary::default();

    for roam in &static_world.animals {
        let animal = &roam.animal;
        let paths = every_model_path(&animal.path, &animal.range);

        for (i, path) in paths.iter().enumerate() {
            let animations = animal.animations.iter().enumerate().map(|(i, _)| {
                asset_server.load(GltfAssetLabel::Animation(i).from_asset(path.clone()))
            });

            let (graph, node_animations) = AnimationGraph::from_clips(animations);

            let nodes = animal
                .animations
                .iter()
                .enumerate()
                .map(|(i, state)| (*state, node_animations[i]))
                .collect();

            library.animals.insert(
                path.clone(),
                AnimalAnimations {
                    graph: graphs.add(graph),
                    nodes,
                },
            );
        }
    }

    commands.insert_resource(library);
}
