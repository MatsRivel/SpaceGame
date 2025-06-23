use bevy::prelude::*;

use crate::{entities::{player::{spawn_player, PlayerTag}, thrusters::{HasThrusters, Thrusters}}, utillity::wrap_map::world_wrap_position, PLAYER_THRUSTER_STRENGTH};
pub struct WorldWrapPlugin;
impl Plugin for WorldWrapPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, world_wrap_position);
    }
}