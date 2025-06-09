use bevy::prelude::*;
use rand::Rng;
use crate::movement::velocity::linear_velocity::Velocity;
use crate::movement::{gravity::gravity_2d::GravityAffected};
use crate::movement::linear_movement_2d::LinearSpeedModifier;
use crate::{ASTEROID_SPEED_MODIFIER};
use crate::entities::object::Object;
#[derive(Component, Default)]
#[require(Object, GravityAffected)]
pub struct Asteroid;

pub fn spawn_asteroides(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\Asteroids\med-a.png";
    let image = asset_server.load(asset_path);
    for i in 0..10{
        commands.spawn((
            Asteroid,
            Sprite::from_image(image.clone()),
            Transform::from_translation(Vec3::new(-200.0 + i as f32 * 50.0, (-1.0 + 2.0*((i%2) as f32)) * 50.0, 0.0)),
            LinearSpeedModifier::new(ASTEROID_SPEED_MODIFIER)
        ));
    }
}
pub fn initialize_asteroide_veloccity(mut query: Query<(&mut Velocity, &LinearSpeedModifier), With<Asteroid>>){
    let mut rng = rand::rng();
    for (mut vel, &modifier) in query.iter_mut(){
        let x = rng.random_range(-1.0..1.0);
        let y = rng.random_range(-1.0..1.0);
        *vel = Velocity::new(vec2(x, y) * modifier);
    }
}