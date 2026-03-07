use bevy::prelude::*;

use crate::infra::components::Infra;

pub fn spawn_infras(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    infras: Vec<Infra>,
) {
    for infra in infras {
        let path = format!("models/infra/{}.glb", infra.path);
        let transform = infra.transform.clone();

        commands.spawn((
            infra,
            transform,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
        ));
    }
}
