use bevy::prelude::*;
use std::ops::Mul;
#[derive(Component,Clone, Copy)]
pub struct LinearSpeedModifier(f32);
impl LinearSpeedModifier{
    pub fn new(speed: f32)->Self{
        Self(speed)
    }
}
impl Default for LinearSpeedModifier{
    fn default() -> Self {
        Self(1.0)
    }
}
impl std::ops::Deref for LinearSpeedModifier {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul<LinearSpeedModifier> for Vec2{
    type Output = Vec2;

    fn mul(self, rhs: LinearSpeedModifier) -> Self::Output {
        self * *rhs
    }
}
impl Mul<LinearSpeedModifier> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: LinearSpeedModifier) -> Self::Output {
        self * *rhs
    }
}