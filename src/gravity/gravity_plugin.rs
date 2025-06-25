use bevy::prelude::*;

use crate::gravity::gravity_2d::{build_gravity_function, crush_when_inside_event_horizon, event_horizon_entry_event, gravity_calculation_flat_true, EnteredEventHorizon};
pub const GRAVITY_FUNC: fn(&Vec2, f32, f32, &Vec2, f32, f32) -> Vec2 = gravity_calculation_flat_true;
pub struct GravityPlugin;
impl Plugin for GravityPlugin{
    fn build(&self, app: &mut App) {
        app.add_event::<EnteredEventHorizon>()
        .add_systems(FixedPreUpdate, build_gravity_function(GRAVITY_FUNC))
        .add_systems(Update,
            (
                event_horizon_entry_event,
                crush_when_inside_event_horizon
            ))
            .add_systems(FixedPostUpdate, build_gravity_function(GRAVITY_FUNC));
    }

}