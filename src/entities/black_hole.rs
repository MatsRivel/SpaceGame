use bevy::prelude::*;
use crate::movement::gravity::gravity_2d::{GravityProducer, Mass};
use crate::movement::linear_movement_2d::LinearSpeedModifier;
use crate::entities::object::Object;
use crate::GRAVITY_WELL_STRENGTH;
#[derive(Component, Default)]
#[require(Object, GravityProducer)]
pub struct GravityWell;

pub fn spawn_gravity_well(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\FX\Bullet-c\bullet-c1.png";
    let image = asset_server.load(asset_path);
    commands.spawn((
        GravityWell,
        GravityProducer::new(GRAVITY_WELL_STRENGTH),
        Sprite::from_image(image.clone()),
        Transform::from_translation(Vec3::new(-300.0, -300.0, 0.0)),
        LinearSpeedModifier::new(0.0),
        Mass::new(100.0)
    ));
}