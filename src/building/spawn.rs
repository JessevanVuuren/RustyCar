use bevy::prelude::*;

use crate::building::components::Building;

pub fn spawn_buildings(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    buildings: Vec<Building>,
) {
    for building in buildings {
        let path = format!("models/buildings/{}.glb", building.path);
        let transform = building.transform.clone();

        commands.spawn((
            building,
            transform,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path))),
        ));
    }
}
