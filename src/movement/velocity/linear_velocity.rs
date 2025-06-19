use bevy::{math::f32, prelude::*};
use std::ops::{Add, AddAssign, Mul};
use crate::{movement::linear_movement_2d::LinearSpeedModifier, MAXIMUM_LINEAR_STEP_LENGTH};

#[derive(Component, Default, Debug, Clone, Copy)]
#[require(LinearSpeedModifier)]
pub struct Velocity(Vec2);

impl Velocity{
    pub fn new(velocity: Vec2)->Self{
        Self(velocity)
    }
    pub fn limit(&mut self, limit: f32){
        let speed = self.0.length();
        if speed > limit{
            if self.0 == Vec2::ZERO { return;}
            self.0 = self.0.normalize() * limit;
        }
    }
}
impl std::ops::Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AddAssign<Vec2> for Velocity{
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs;
    }
}
impl Add<Vec2> for Velocity{
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl Into<Vec3> for Velocity{
    fn into(self) -> Vec3 {
        self.0.extend(0.0)
    }
}

pub fn conservation_of_linear_momentum(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &LinearSpeedModifier)>
){
    for (mut transform, &velocity_2d, &speed_mod) in query.iter_mut() {
        let adjusted_speed = *speed_mod * time.delta_secs();
        let velocity_3d: Vec3 = velocity_2d.into();
        let velocity_adjusted = velocity_3d * adjusted_speed;
        let limited_velocity = match velocity_adjusted.length() >  MAXIMUM_LINEAR_STEP_LENGTH{
            true => velocity_adjusted.normalize() * MAXIMUM_LINEAR_STEP_LENGTH,
            false => velocity_adjusted,
        };
        transform.translation += limited_velocity;
    }
}