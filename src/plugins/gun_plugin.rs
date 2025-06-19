use bevy::prelude::*;

use crate::{entities::{gun::fire_bullet, player::{give_player_gun, spawn_player}}, movement::velocity::throttle_velocity::throttle_bullet_velocity, utillity::timing::self_destruct_countdown};
pub struct GunPlugin;
impl Plugin for GunPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, 
            give_player_gun.after(spawn_player)
        ).add_systems(Update, (
            throttle_bullet_velocity,
            fire_bullet, 
            self_destruct_countdown));
    }

}