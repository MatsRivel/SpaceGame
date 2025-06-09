use crate::entities::gun::Gun;
use crate::movement::rotational_movement_2d::RotationalSpeedModifier;
use crate::movement::linear_movement_2d::LinearSpeedModifier;
use crate::movement::gravity::gravity_2d::GravityAffected;
use crate::movement::velocity::angular_velocity::AngularVelocity;
use crate::movement::velocity::linear_velocity::Velocity;
use crate::{PLAYER_ROT_SPEED_MODIFIER, PLAYER_SPEED_MODIFIER};
use crate::entities::object::Object;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Object, GravityAffected, RotationalSpeedModifier, AngularVelocity)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\Ships\ship-a\ship-a1.png";
    let image = asset_server.load(asset_path);
    commands.spawn((
        Player,
        Sprite::from_image(image),
        LinearSpeedModifier::new(PLAYER_SPEED_MODIFIER),
        RotationalSpeedModifier::new(PLAYER_ROT_SPEED_MODIFIER)
    ));
}

pub fn spawn_player_with_gun(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\Ships\ship-a\ship-a1.png";
    let image = asset_server.load(asset_path);
    commands.spawn((
        Player,
        Sprite::from_image(image),
        LinearSpeedModifier::new(PLAYER_SPEED_MODIFIER),
        RotationalSpeedModifier::new(PLAYER_ROT_SPEED_MODIFIER),
        Gun::new(1.0, 5, 5)
    ));
}

pub fn accelerate_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&Transform, &mut Velocity), With<Player>>,
){
    let (transform, mut velocity) = query.into_inner();
    let left = keyboard_input.pressed(KeyCode::KeyQ);
    let right = keyboard_input.pressed(KeyCode::KeyE);
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let sideways_momentum = match (left,right){
        (true,false) => -1.0,
        (false,true) => 1.0,
        (true,true) | (false,false) => 0.0
    };
    let forwards_momentum = match (up,down){
        (true,false) => 1.0,
        (false,true) => -1.0,
        (true,true) | (false,false) => 0.0
    };
    let momentum = vec2(sideways_momentum,forwards_momentum);
    if momentum == Vec2::ZERO{
        return;
    }
    let rotation_adjusted_movement = transform.rotation.mul_vec3(momentum.extend(0.0));
    let delta_time_movement = rotation_adjusted_movement*time.delta_secs();
    *velocity += delta_time_movement.truncate();
}

pub fn rotate_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&mut AngularVelocity, &RotationalSpeedModifier), With<Player>>,
){
    let (mut angular_velocity, rotational_modifier) = query.into_inner();
    let clockwise = keyboard_input.pressed(KeyCode::KeyD);
    let counterwise = keyboard_input.pressed(KeyCode::KeyA);
    let rotary_momentum = match (counterwise, clockwise){
        (true,false) => 1.0,
        (false,true) => -1.0,
        (true,true) | (false,false) => 0.0
    };
    
    *angular_velocity += rotational_modifier * rotary_momentum * time.delta_secs();
}