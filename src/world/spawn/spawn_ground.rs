use crate::world::{
    components::{Ground, GroundId, StaticWorld, TileType, TileWorld},
    ground::ground::ground_plane,
    utils::range_from_surfaces,
};
use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

pub fn spawn_ground(
    mut commands: Commands,
    static_world: Res<StaticWorld>,
    mut world: ResMut<TileWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = SmallRng::seed_from_u64(1604);

    for (layer, block) in static_world.blocks.iter().enumerate() {
        if let TileType::Ground(ground) = &block.tiletype {
            let range = range_from_surfaces(&block.surface);

            for tile in range {
                let mut pos = tile.to_world_transform();
                let grass = ground_plane(&mut rng, pos.translation, &ground);

                let id = spawn_grass(grass, &pos, &mut commands, &mut meshes, &mut materials);

                world.ground.insert(
                    tile,
                    GroundId {
                        entity: id,
                        id: layer + 1,
                    },
                );
            }
        }
    }
}

fn spawn_grass(
    grass: Mesh,
    transform: &Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    commands
        .spawn((
            Ground,
            Mesh3d(meshes.add(grass)),
            MeshMaterial3d(materials.add(Color::WHITE)),
            transform.clone(),
        ))
        .id()
}
