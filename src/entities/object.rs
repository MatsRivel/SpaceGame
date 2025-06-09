use bevy::prelude::*;
use crate::movement::velocity::linear_velocity::Velocity;

#[derive(Component,Default)]
#[require(Transform, Sprite, Velocity)]
pub struct Object;