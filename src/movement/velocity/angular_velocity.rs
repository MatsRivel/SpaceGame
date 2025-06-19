use bevy::prelude::*;
use std::ops::{AddAssign, Mul};
use crate::movement::rotational_movement_2d::RotationalSpeedModifier;

#[derive(Component, Default, Debug, Clone, Copy)]
#[require(RotationalSpeedModifier)]
pub struct AngularVelocity(f32);

impl AngularVelocity{
    pub fn new(velocity: f32)->Self{
        Self(velocity)
    }
}
impl std::ops::Deref for AngularVelocity {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl AddAssign<f32> for AngularVelocity{
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
    }
}
pub fn apply_angular_velocity_to_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &AngularVelocity,  &RotationalSpeedModifier)>,
){
    for (mut transform, &angular_velocity, &speed_mod) in query.iter_mut() {
        let angular_speed = *angular_velocity * *speed_mod;
        let delta_angle = angular_speed * time.delta_secs();
        let delta_rotation = Quat::from_rotation_z(delta_angle);
        transform.rotation *= delta_rotation;
    }
}