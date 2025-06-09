use bevy::prelude::*;
use std::ops::Mul;

#[derive(Component,Default,Copy,Clone)]
pub struct RotationalSpeedModifier(f32);

impl RotationalSpeedModifier{
    pub fn new(speed: f32)->Self{
        Self(speed)
    }
}

impl std::ops::Deref for RotationalSpeedModifier {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul<f32> for &RotationalSpeedModifier{
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}
impl Mul<RotationalSpeedModifier> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: RotationalSpeedModifier) -> Self::Output {
        self * *rhs
    }
}