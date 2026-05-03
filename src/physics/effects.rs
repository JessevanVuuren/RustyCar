use bevy::{math::NormedVectorSpace, prelude::*};

use crate::physics::components::{Collision, Effect, Velocity};

pub fn collision_effect_response(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Collision, &mut Velocity)>,
) {
    for (entity, mut transform, collision, mut velocity) in query {
        if matches!(collision.effect, Effect::InverseVelocity) {
            transform.translation += collision.normal * collision.depth;

            let vn = velocity.0.dot(collision.normal);

            if vn < 0.0 {
                velocity.0 = velocity.0 - 2.0 * vn * collision.normal;
            }

            commands.entity(entity).remove::<Collision>();
        }
    }
}
