use bevy::prelude::*;

use crate::{
    components::{AffectedByGravity, Velocity},
    Gravity, BASE_SPEED,
};

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gravity_system);
    }
}

fn gravity_system(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Velocity, &AffectedByGravity), With<AffectedByGravity>>,
) {
    for (mut velocity, affected_by_gravity) in query.iter_mut() {
        if affected_by_gravity.is_affected {
            velocity.y -= gravity.amplitude * time.delta_seconds() * BASE_SPEED;
        } else {
            velocity.y = 0.;
        }
    }
}
