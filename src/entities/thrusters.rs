use bevy::prelude::*;
use crate::movement::velocity::{linear_acceleration::LinearAcceleration};
#[derive(Component, Default)]
#[require(LinearAcceleration)]
pub struct Thrusters(pub f32);
impl std::ops::Deref for Thrusters {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
