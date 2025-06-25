use bevy::prelude::*;
use crate::{bullet::bullet_factory::BulletFactory, entities::player::{spawn_player, PlayerTag}, gun::{fire_bullet, give_player_gun}, movement::velocity::throttle_velocity::throttle_bullet_velocity, utillity::timing::self_destruct_countdown};

pub struct GunPlugin;
impl Plugin for GunPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, 
            give_player_gun::<PlayerTag, BulletFactory>.after(spawn_player)
        ).add_systems(Update, (
            throttle_bullet_velocity,
            fire_bullet::<PlayerTag, BulletFactory>, 
            self_destruct_countdown));
    }
}