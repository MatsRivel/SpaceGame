use bevy::prelude::*;
use crate::camera::following_camera::{make_camera_follow, move_following_camera};
use crate::camera::{apply_camera_zoom, spawn_camera};
use crate::entities::gravity_well::spawn_gravity_well;

use crate::entities::player::{accelerate_player,  rotate_player, spawn_player, PlayerTag};
use crate::movement::velocity::angular_velocity::apply_angular_velocity_to_position;
use crate::movement::velocity::linear_acceleration::apply_linear_acceleration;
use crate::movement::velocity::linear_velocity::apply_linear_velocity_to_position;
use crate::entities::asteroid::{
        initialize_asteroide_veloccity, spawn_asteroides, spawn_friendly_asteroide, DestroyAsteroid
    };
use crate::movement::velocity::throttle_velocity::{
    throttle_asteroid_velocity, 
    throttle_player_velocity};
    
#[cfg(debug_assertions)]
use crate::utillity::gizmos::{draw_gravity_falloff,draw_arrow, draw_player_trajectory, to_well, MyArrowGizmos};
use crate::TRAJECTORY_LENGTH;

pub fn add_camera(app: &mut App){
    app.add_systems(Startup,    (
        spawn_camera, 
        make_camera_follow::<PlayerTag>.after(spawn_camera),
        apply_camera_zoom.after(spawn_camera)
    )).add_systems(Update, move_following_camera);
}

pub fn add_gizmos(app: &mut App){
    app.init_gizmo_group::<MyArrowGizmos>()
        .add_systems(Update, (
            draw_arrow,
            to_well,
            draw_gravity_falloff,
            draw_player_trajectory::<TRAJECTORY_LENGTH>
        ));
}
pub fn add_movement(app: &mut App){
        app.add_systems(FixedUpdate, (
            apply_linear_velocity_to_position, 
            apply_angular_velocity_to_position
        )).add_systems(Update, apply_linear_acceleration);
}
pub fn add_player(app: &mut App){
        app.add_systems(Startup, 
            spawn_player)
        .add_systems(Update, (
            rotate_player, 
            accelerate_player, 
            throttle_player_velocity
        ));
}

pub fn add_asteroid(app: &mut App){
    app.add_systems(Startup,(
            // spawn_asteroides,
            spawn_friendly_asteroide,
            initialize_asteroide_veloccity.after(spawn_asteroides)
        )).add_systems(Update, (
            throttle_asteroid_velocity,
        )).add_event::<DestroyAsteroid>();
}
pub fn add_gravity_well(app: &mut App){
    app.add_systems(Startup,spawn_gravity_well);
}
