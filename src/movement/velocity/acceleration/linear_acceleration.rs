use std::ops::{Add, AddAssign, MulAssign};

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
    #[allow(unused)]
    pub fn limit(&mut self, limit: f32){
        let acc = self.0.length();
        if acc > limit{
            if self.0 == Vec2::ZERO { return;}
            self.0 = self.0.normalize() * limit;
        }
    }
    #[allow(unused)]
    pub fn add(&mut self, value: Vec2){
        self.0 += value;
    }
    pub fn clear(&mut self){
        self.0 += Vec2::ZERO;
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
impl MulAssign<f32> for LinearAcceleration{
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}
impl Add<Vec2> for LinearAcceleration{
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}
impl From<LinearAcceleration> for Vec3{
    fn from(value: LinearAcceleration) -> Self {
        value.0.extend(0.0)
    }
}

impl AddAssign<LinearAcceleration> for Velocity{
    fn add_assign(&mut self, rhs: LinearAcceleration) {
        self.add(rhs.0);
    }
}

pub fn apply_linear_acceleration(time: Res<Time>, mut query: Query<(&mut Velocity, &mut LinearAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += *acc;
        acc.clear();
    }
}