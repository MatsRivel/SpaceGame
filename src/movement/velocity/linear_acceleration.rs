use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use crate::movement::linear_movement_2d::LinearSpeedModifier;
use crate::movement::velocity::linear_velocity::Velocity;

#[derive(Component, Default, Debug, Clone, Copy)]
#[require(Velocity, LinearSpeedModifier)]
pub struct LinearAcceleration(Vec2);
impl LinearAcceleration{
pub fn new(acceleration: Vec2)->Self{
    Self(acceleration)
}
    pub fn limit(&mut self, limit: f32){
        let acc = self.0.length();
        if acc > limit{
            if self.0 == Vec2::ZERO { return;}
            self.0 = self.0.normalize() * limit;
        }
    }
}
impl std::ops::Deref for LinearAcceleration {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AddAssign<Vec2> for LinearAcceleration{
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs;
    }
}
impl Add<Vec2> for LinearAcceleration{
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl Into<Vec3> for LinearAcceleration{
    fn into(self) -> Vec3 {
        self.0.extend(0.0)
    }
}