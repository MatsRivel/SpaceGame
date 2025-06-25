use bevy::prelude::*;
use rand::Rng;
use crate::destruction::{destroy_asteroid, destroy_destructible, Destructible};
use crate::gravity::gravity_2d::{GravityAffected, Mass};
use crate::movement::velocity::linear_velocity::Velocity;
use crate::movement::linear_movement_2d::LinearSpeedModifier;
use crate::{ASTEROID_SPEED_MODIFIER};
use crate::entities::object::Object;

#[derive(Component, Default)]
#[require(Object, GravityAffected)]
pub struct Asteroid;

pub fn spawn_asteroides(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\Asteroids\med-a.png";
    let image = asset_server.load(asset_path);
    let mut rng = rand::rng();
    let mut sprite = Sprite::from_image(image.clone());
    sprite.custom_size = Some(Vec2::new(128.0, 128.0));
    let sprite = sprite;
    for _ in 0..100{
        commands.spawn((
            Asteroid,
            sprite.clone(),
            Transform::from_translation(Vec3::new(rng.random_range(-1.0..1.0) * 5000.0 , 3000.0* rng.random_range(-1.0..1.0), 0.0)),
            LinearSpeedModifier::new(ASTEROID_SPEED_MODIFIER*rng.random_range(1.0..10.0)),
            Mass::new(1.0),
        ));
    }
}


pub fn spawn_friendly_asteroide(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\Asteroids\big-a.png";
    let image = asset_server.load(asset_path);
    let mut sprite = Sprite::from_image(image.clone());
    sprite.custom_size = Some(Vec2::new(256.0, 256.0));
    let sprite = sprite;
    let entity = commands.spawn((
        Asteroid,
        sprite.clone(),
        Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
        LinearSpeedModifier::new(0.0),
        Mass::new(1.0),
        Destructible
    )).id();
    // Create a global observer watching this entity. Triggers only for this entity.
    let mut observer = Observer::new(destroy_asteroid);
    observer.watch_entity(entity);
    commands.spawn(observer); // <- THIS is what hooks it all up
}


pub fn initialize_asteroide_veloccity(mut query: Query<(&mut Velocity, &LinearSpeedModifier), With<Asteroid>>){
    let mut rng = rand::rng();
    for (mut vel, &modifier) in query.iter_mut(){
        let x = rng.random_range(-1.0..1.0);
        let y = rng.random_range(-1.0..1.0);
        *vel = Velocity::new(vec2(x, y) * modifier);
    }
}