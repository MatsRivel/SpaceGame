use bevy::prelude::*;

use crate::{entities::object::Object, movement::gravity::gravity_2d::GravityAffected, utillity::timing::SelfDestructTimer};

#[derive(Component)]
#[require(Object, GravityAffected, SelfDestructTimer)]
pub struct Bullet{
    pub damage: f32,
    pub radius: f32,
    pub mass: f32,
}
impl Bullet{
    pub fn new(damage: f32,radius: f32,mass: f32) -> Self{
        Self { damage, radius, mass}
    }
}