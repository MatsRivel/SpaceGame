use bevy::prelude::*;
use crate::movement::velocity::{acceleration::linear_acceleration::LinearAcceleration};
pub mod thrusters_plugin;

#[derive(Component, Debug)]
pub struct HasThrusters;

#[derive(Component, Default, Clone, Copy)]
#[require(LinearAcceleration, Sprite)]
pub struct Thrusters(pub f32);
impl std::ops::Deref for Thrusters {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
