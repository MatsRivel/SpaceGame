use bevy::prelude::*;
use std::ops::Mul;

use crate::{entities::player::PlayerTag, movement::velocity::{acceleration::linear_acceleration::apply_linear_acceleration, angular_velocity::{apply_angular_velocity_to_position, AngularVelocity}, linear_velocity::{apply_linear_velocity_to_position, Velocity}}};
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

pub struct LinearMovement2DPlugin;
impl Plugin for LinearMovement2DPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            apply_linear_velocity_to_position, 
            apply_angular_velocity_to_position
        )).add_systems(Update, apply_linear_acceleration);
    }
}
pub struct ZeroVelocityWhenNoInputPlugin;
impl Plugin for ZeroVelocityWhenNoInputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (
            set_linear_velocity_to_zero::<PlayerTag>,
            set_angular_velocity_to_zero::<PlayerTag>
        ));
    }
}
pub fn set_linear_velocity_to_zero<T:Component>(mut query: Query<&mut Velocity, With<T>>){
    for mut q in query.iter_mut(){
        *q = Velocity::new(Vec2::ZERO);
    }
}
pub fn set_angular_velocity_to_zero<T:Component>(mut query: Query<&mut AngularVelocity, With<T>>){
    for mut q in query.iter_mut(){
        *q = AngularVelocity::new(0.0);
    }
}