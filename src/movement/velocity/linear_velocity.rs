use bevy::{math::f32, prelude::*};
use std::ops::{Add, AddAssign, Mul};
use crate::movement::linear_movement_2d::LinearSpeedModifier;

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

impl Mul<LinearSpeedModifier> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: LinearSpeedModifier) -> Self::Output {
        self * *rhs
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
    for (mut transform, &velocity, &speed_mod) in query.iter_mut() {
        let direction_adjusted_speed: Vec3 = velocity.into();
        transform.translation += direction_adjusted_speed * speed_mod * time.delta_secs();
    }
}