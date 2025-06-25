use bevy::prelude::*;
use crate::bullet::bullet::Bullet;
use crate::entities::asteroid::Asteroid;
use crate::entities::player::PlayerTag;
use crate::movement::{velocity::linear_velocity::Velocity};
use crate::MAXIMUM_LINEAR_PLAYER_SPEED;
use crate::MAXIMUM_LINEAR_ASTEROID_SPEED;
use crate::MAXIMUM_LINEAR_BULLET_SPEED;

pub fn throttle_player_velocity(mut query: Query<&mut Velocity, With<PlayerTag>>){
    for mut velocity in query.iter_mut() {
        velocity.limit(MAXIMUM_LINEAR_PLAYER_SPEED);
    }   
}
pub fn throttle_asteroid_velocity(mut query: Query<&mut Velocity, With<Asteroid>>){
    for mut velocity in query.iter_mut() {
        velocity.limit(MAXIMUM_LINEAR_ASTEROID_SPEED);
    }   
}
pub fn throttle_bullet_velocity(mut query: Query<&mut Velocity, With<Bullet>>){
    for mut velocity in query.iter_mut() {
        velocity.limit(MAXIMUM_LINEAR_BULLET_SPEED);
    }   
}