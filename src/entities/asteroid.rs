use bevy::prelude::*;
use rand::Rng;
use crate::movement::gravity::gravity_2d::Mass;
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
    let mut rng = rand::rng();
    let mut observer = Observer::new(destory_asteroide);
    for _ in 0..100{
        let entity = commands.spawn((
            Asteroid,
            Sprite::from_image(image.clone()),
            Transform::from_translation(Vec3::new(rng.random_range(-1.0..1.0) * 5000.0 , 3000.0* rng.random_range(-1.0..1.0), 0.0)),
            LinearSpeedModifier::new(ASTEROID_SPEED_MODIFIER*rng.random_range(1.0..10.0)),
            Mass::new(1.0),
        )).observe(destory_asteroide).id();
        observer.watch_entity(entity);
    }
    commands.spawn(observer);

}
#[derive(Event)]
pub struct DestoryAsteroide;
pub fn destory_asteroide(trigger: Trigger<DestoryAsteroide>, mut commands: Commands, asset_server: Res<AssetServer>){
    dbg!("Asteroide destoryed!");
    let id = trigger.target();
        let Ok(mut entity) = commands.get_entity(id) else {
        return;
    };
    entity.despawn();
    // This below should be brough into a func...
    let asset_path = r"sprites\Asteroids\med-a.png";
    let image = asset_server.load(asset_path);
    let mut rng = rand::rng();
    let mut observer = Observer::new(destory_asteroide);
    
    let entity = commands.spawn((
        Asteroid,
        Sprite::from_image(image.clone()),
        Transform::from_translation(Vec3::new(rng.random_range(-1.0..1.0) * 5000.0 , 3000.0* rng.random_range(-1.0..1.0), 0.0)),
        LinearSpeedModifier::new(ASTEROID_SPEED_MODIFIER*rng.random_range(1.0..10.0)),
        Mass::new(100_000.0),
    )).observe(destory_asteroide).id();
    observer.watch_entity(entity);
    commands.spawn(observer);
}

pub fn initialize_asteroide_veloccity(mut query: Query<(&mut Velocity, &LinearSpeedModifier), With<Asteroid>>){
    let mut rng = rand::rng();
    for (mut vel, &modifier) in query.iter_mut(){
        let x = rng.random_range(-1.0..1.0);
        let y = rng.random_range(-1.0..1.0);
        *vel = Velocity::new(vec2(x, y) * modifier);
    }
}