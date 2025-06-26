use bevy::prelude::*;
use crate::gravity::gravity_2d::{GravityProducer, Mass};
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
        Transform::from_translation(Vec3::new(-5000.0, -3000.0, 0.0)),
        LinearSpeedModifier::new(0.0),
        Mass::new(1.0)
    ));
}

pub struct GravityWellPlugin;
impl Plugin for GravityWellPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_gravity_well);
    }
}