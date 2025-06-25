use bevy::prelude::*;
use crate::{destruction::Destroying, entities::object::Object, gravity::gravity_2d::GravityAffected, utillity::timing::SelfDestructTimer};
pub mod bullet_maker_trait;
pub mod bullet_factory;

#[allow(unused)] // We allow some fields to be unused, as damage mechanics does not exist yet
#[derive(Component)]
#[require(Object, GravityAffected, SelfDestructTimer, Destroying)]
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