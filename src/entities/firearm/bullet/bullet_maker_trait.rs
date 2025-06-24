use bevy::prelude::*;

use crate::{entities::firearm::bullet::bullet::Bullet, movement::velocity::linear_velocity::Velocity, utillity::timing::SelfDestructTimer};
pub trait MakeBullet{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_position: &Vec2, adjusted_rotation: &Quat)->(Bullet, Velocity, Transform, Sprite, SelfDestructTimer);
}