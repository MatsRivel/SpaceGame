use bevy::prelude::*;
use crate::camera::following_camera::{make_camera_follow, move_following_camera};
use crate::camera::{apply_camera_zoom, spawn_camera};
use crate::entities::gravity_well::spawn_gravity_well;
use crate::entities::gun::fire_bullet;
use crate::entities::player::{accelerate_player, give_player_gun, give_player_thrusters, rotate_player, spawn_player, PlayerTag};
use crate::movement::gravity::gravity_2d::{build_gravity_function, crush_when_inside_event_horizon, event_horizon_entry_event, gravity_calculation_flat_saviour, gravity_calculation_flat_true, gravity_calculation_true, EnteredEventHorizon};
use crate::movement::velocity::angular_velocity::conservation_of_angular_momentum;
use crate::movement::velocity::linear_velocity::conservation_of_linear_momentum;
use crate::entities::asteroid::{
        initialize_asteroide_veloccity, spawn_asteroides
    };
use crate::movement::velocity::throttle_velocity::{
    throttle_asteroid_velocity, 
    throttle_bullet_velocity, 
    throttle_player_velocity};
use crate::utillity::timing::self_destruct_countdown;
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
pub const GRAVITY_FUNC: fn(&Vec2, f32, f32, &Vec2, f32, f32) -> Vec2 = gravity_calculation_flat_true;

pub fn add_gravity(app: &mut App){
    app.add_event::<EnteredEventHorizon>()
    .add_systems(FixedPreUpdate, build_gravity_function(GRAVITY_FUNC))
    .add_systems(Update,
        (
            event_horizon_entry_event,
            crush_when_inside_event_horizon
        ))
        .add_systems(FixedPostUpdate, build_gravity_function(GRAVITY_FUNC));
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
pub fn add_gun(app: &mut App){
        app.add_systems(Startup, 
            give_player_gun.after(spawn_player)
        ).add_systems(Update, (
            throttle_bullet_velocity,
            fire_bullet, 
            self_destruct_countdown));
}
pub fn add_movement(app: &mut App){
        app.add_systems(FixedUpdate, (
            conservation_of_linear_momentum, 
            conservation_of_angular_momentum));
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
pub fn add_player_thrusters( app: &mut App){
        app.add_systems(Startup, 
            give_player_thrusters.after(spawn_player));
}
pub fn add_asteroid(app: &mut App){
    app.add_systems(Startup,(
            spawn_asteroides,
            initialize_asteroide_veloccity.after(spawn_asteroides)
        )).add_systems(Update, throttle_asteroid_velocity);
}
pub fn add_gravity_well(app: &mut App){
    app.add_systems(Startup,spawn_gravity_well);
}
