use bevy::prelude::*;
use crate::{bullet::{bullet_factory::BulletFactory, Bullet}, entities::player::{spawn_player, PlayerTag}, gun::{fire_bullet, give_player_gun}, movement::velocity::throttle_velocity::throttle_velocity, utillity::timing::self_destruct_countdown, MAXIMUM_LINEAR_BULLET_SPEED};

pub struct GunPlugin;
impl Plugin for GunPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, 
            give_player_gun::<PlayerTag, BulletFactory>.after(spawn_player)
        ).add_systems(Update, (
            throttle_velocity::<Bullet,MAXIMUM_LINEAR_BULLET_SPEED>,
            fire_bullet::<PlayerTag, BulletFactory>, 
            self_destruct_countdown));
    }
}

