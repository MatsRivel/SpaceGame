use bevy::prelude::*;
use crate::movement::{velocity::linear_velocity::Velocity};

pub fn throttle_velocity<T: Component, const N: u32>(mut query: Query<&mut Velocity, With<T>>){
    for mut velocity in query.iter_mut() {
        velocity.limit(N as f32);
    }   
}
