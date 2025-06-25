use bevy::prelude::*;

use crate::utillity::wrap_map::world_wrap_position;

pub struct WorldWrapPlugin;
impl Plugin for WorldWrapPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, world_wrap_position);
    }
}