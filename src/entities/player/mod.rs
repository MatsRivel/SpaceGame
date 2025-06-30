use crate::gravity::gravity_2d::GravityAffected;
use crate::movement::rotational_movement_2d::RotationalSpeedModifier;
use crate::movement::linear_movement_2d::{LinearSpeedModifier};
use crate::movement::velocity::angular_velocity::{apply_angular_velocity_to_position, AngularVelocity};
use crate::movement::velocity::acceleration::linear_acceleration::{LinearAcceleration};
use crate::movement::velocity::linear_velocity::apply_linear_velocity_to_position;
use crate::movement::velocity::throttle_velocity::throttle_velocity;
use crate::thrusters::{HasThrusters, Thrusters};
use crate::{MAXIMUM_LINEAR_PLAYER_SPEED, PLAYER_BODY_IMAGE_PATH, PLAYER_ROT_SPEED_MODIFIER, PLAYER_SPEED_MODIFIER};
use crate::entities::object::Object;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Object, GravityAffected, RotationalSpeedModifier, AngularVelocity)]
pub struct PlayerTag;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    // let asset_path = r"sprites\Ships\ship-a\ship-a1.png";
    let image = asset_server.load(PLAYER_BODY_IMAGE_PATH);
    let mut sprite =     Sprite::from_image(image);
    sprite.custom_size = Some(Vec2::splat(128.0));
    let player_entity = commands.spawn((
        PlayerTag,
        sprite,
        LinearSpeedModifier::new(PLAYER_SPEED_MODIFIER),
        RotationalSpeedModifier::new(PLAYER_ROT_SPEED_MODIFIER),
    )).id();
    println!("Player Entity: {player_entity:?}");
}

fn get_thrust(possible_children: Option<&Children>, possible_thrusters: Option<&HasThrusters>, thruster_query: Query<&Thrusters>)->f32{
    if possible_thrusters.is_some() && let Some(children) = possible_children{
        children
            .iter()
            .filter_map(|child| thruster_query.get(child).ok())
            .map(|thruster| **thruster)
            .sum()
    }else{
        1.0
    }
}

#[allow(clippy::type_complexity)] // Does not make sense to pull these from the query.
pub fn apply_acceleration_to_single_from_keyboard<T:Component>(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &mut LinearAcceleration, Option<&Children>, Option<&HasThrusters>), With<T>>,
    thruster_query: Query<&Thrusters>
){
    let (transform, mut acceleration, possible_children, possible_thrusters) = player_query.into_inner();
    let thrust = get_thrust(possible_children, possible_thrusters, thruster_query);
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
    let rotation_adjusted_movement = transform.rotation.mul_vec3(momentum.extend(0.0)).truncate();
    let delta_time_movement = rotation_adjusted_movement*time.delta_secs()*thrust;
    *acceleration += delta_time_movement;
}

pub fn apply_acceleration_rotation_velocity<T:Component>(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&mut AngularVelocity, &RotationalSpeedModifier), With<T>>,
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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, 
            spawn_player)
        .add_systems(Update, (
            apply_acceleration_rotation_velocity::<PlayerTag>,
            apply_acceleration_to_single_from_keyboard::<PlayerTag>,
            throttle_velocity::<PlayerTag,MAXIMUM_LINEAR_PLAYER_SPEED>
        ));
    }
}