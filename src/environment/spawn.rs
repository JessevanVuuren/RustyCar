use bevy::prelude::*;

use crate::environment::components::World;

pub fn spawn_environment(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    world: Vec<World>,
) {
    for object in world {
        let path = format!("models/nature/{}.glb", object.model.name);
        let transform = object.transform.clone();

        commands.spawn((
            object.model,
            transform,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
        ));
    }
}
