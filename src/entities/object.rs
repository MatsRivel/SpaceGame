use bevy::prelude::*;
use crate::movement::velocity::acceleration::linear_acceleration::LinearAcceleration;

#[derive(Component,Default)]
#[require(Transform, Sprite, LinearAcceleration)]
pub struct Object;