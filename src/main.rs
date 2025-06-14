use bevy::prelude::*;
mod movement;
mod entities;
mod utillity;
use crate::entities::black_hole::spawn_gravity_well;
use crate::entities::gun::fire_bullet;
use crate::movement::gravity::gravity_2d::{apply_gravity, crush_when_inside_event_horizon, event_horizon_entry_event, EnteredEventHorizon};
use crate::movement::velocity::angular_velocity::conservation_of_angular_momentum;
use crate::entities::player::{
        accelerate_player, give_player_gun, rotate_player, spawn_player, PlayerTag
    };
use crate::movement::velocity::linear_velocity::conservation_of_linear_momentum;
use crate::entities::asteroid::{
        initialize_asteroide_veloccity, 
        spawn_asteroides
    };
use crate::movement::velocity::throttle_velocity::{
    throttle_asteroid_velocity, 
    throttle_bullet_velocity, 
    throttle_player_velocity};
#[cfg(debug_assertions)]
use crate::utillity::gizmos::draw_gravity_falloff;
use crate::utillity::gizmos::{draw_arrow, to_well, MyArrowGizmos};
use crate::utillity::timing::self_destruct_countdown;


const PLAYER_SPEED_MODIFIER: f32 = 100.0f32;
const PLAYER_ROT_SPEED_MODIFIER: f32 = 2.0f32;
const ASTEROID_SPEED_MODIFIER: f32 = 50.0f32;
const BULLET_SPEED_MODIFIER: f32 = 100.0f32;

const GRAVITY_WELL_STRENGTH: f32 = 0.01f32;

const MAXIMUM_LINEAR_PLAYER_SPEED: f32 = 2.0f32;
const MAXIMUM_LINEAR_ASTEROID_SPEED: f32 = 1.0f32;
const MAXIMUM_LINEAR_BULLET_SPEED: f32 = 5.0f32;

const MAXIMUM_LINEAR_STEP_LENGTH: f32 = 10.0f32;

pub fn add_camera(app: &mut App){
    app.add_systems(Startup,    (
        spawn_camera, 
        make_camera_follow.after(spawn_camera),
        apply_camera_zoom.after(spawn_camera)
    )).add_systems(Update, move_following_camera);
}
pub fn add_gravity(app: &mut App){
    app.add_event::<EnteredEventHorizon>()
        .add_systems(Update,
            (
            apply_gravity,
            event_horizon_entry_event,
            crush_when_inside_event_horizon
    ));
}
pub fn add_gizmos(app: &mut App){
    app.init_gizmo_group::<MyArrowGizmos>()
        .add_systems(Update, (
            draw_arrow,
            to_well,
            draw_gravity_falloff));
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
            throttle_player_velocity));
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
fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vk");
    }
    const SKY_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.5);
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        // primary_window: Some(Window {
        //     resolution: WindowResolution::new(200., 100.).with_scale_factor_override(1.0),
        //     ..Default::default()
        // }),
        ..default()
    }));
    
    app.insert_resource(ClearColor(SKY_COLOR));
    app.insert_resource(Time::<Fixed>::from_hz(30.0)) ;//This messe s with time.
    
    add_camera(&mut app);
    add_player(&mut app);
    add_movement(&mut app);
    add_gravity(&mut app);
    add_gun(&mut app);
    add_asteroid(&mut app);
    add_gravity_well(&mut app);
    #[cfg(debug_assertions)]
    add_gizmos(&mut app);

    app.run();
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
pub fn apply_camera_zoom(
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    for mut projection in query.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = 1.5; // Zoomed out; try 0.5 to zoom in
        }
    }
}
#[derive(Component)]
#[require(Camera2d)]
pub struct FollowingCameraTag;
pub fn make_camera_follow(mut commands: Commands,query: Single<Entity, (With<Camera2d>,Without<FollowingCameraTag>)>){
    commands.entity(query.into_inner()).insert(FollowingCameraTag);

}
pub fn move_following_camera(
    cam_query: Single<&mut Transform, (With<FollowingCameraTag>, Without<PlayerTag>)>,
    player_query: Single<&Transform, (With<PlayerTag>, Without<FollowingCameraTag>)>
){
    let player_pos = player_query.into_inner().translation;
    let mut cam = cam_query.into_inner();
    cam.translation = player_pos;
}





