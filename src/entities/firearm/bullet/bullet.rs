use bevy::prelude::*;

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