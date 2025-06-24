use bevy::prelude::*;
pub trait MakeBullet{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_position: &Vec2, adjusted_rotation: &Quat)->(Bullet, Velocity, bevy::prelude::Transform, bevy::prelude::Sprite, SelfDestructTimer);
}