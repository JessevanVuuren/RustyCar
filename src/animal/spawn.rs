use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::animal::components::AnimationToPlay;

pub const BASE_ASSET: &str = "models/";

pub fn spawn_butterfly(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/animals/butterfly.glb")),
    );

    let graph_handle = graphs.add(graph);

    let mesh_scene = SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/animals/butterfly.glb")),
    );

    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    commands
        .spawn((animation_to_play, mesh_scene))
        .observe(play_animation_when_ready);
}

fn play_animation_when_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(scene_ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                player.play(animation_to_play.index).repeat();

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}