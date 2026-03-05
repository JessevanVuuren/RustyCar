use bevy::prelude::*;

use crate::environment::components::Env;

pub fn spawn_environment(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    world: Vec<Env>,
) {
    for env in world {
        let path = format!("models/nature/{}.glb", env.name);
        let transform = env.transform.clone();

        commands.spawn((
            env,
            transform,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
        ));
    }
}
