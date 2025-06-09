use bevy::prelude::*;
mod movement;
mod entities;
mod utillity;
use crate::entities::gun::fire_bullet;
use crate::entities::object::Object;
use crate::movement::velocity::angular_velocity::conservation_of_angular_momentum;
use crate::entities::player::{
        accelerate_player, 
        rotate_player, 
        spawn_player, 
        spawn_player_with_gun
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
use crate::utillity::gizmos::{draw_arrow, MyArrowGizmos};
use crate::utillity::timing::self_destruct_countdown;

const PLAYER_SPEED_MODIFIER: f32 = 100.0f32;
const PLAYER_ROT_SPEED_MODIFIER: f32 = 2.0f32;
const ASTEROID_SPEED_MODIFIER: f32 = 100.0f32;

const MAXIMUM_LINEAR_PLAYER_SPEED: f32 = 2.0f32;
const MAXIMUM_LINEAR_ASTEROID_SPEED: f32 = 1.0f32;
const MAXIMUM_LINEAR_BULLET_SPEED: f32 = 5.0f32;

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vk");
    }
    const SKY_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.5);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // primary_window: Some(Window {
            //     resolution: WindowResolution::new(200., 100.).with_scale_factor_override(1.0),
            //     ..Default::default()
            // }),
            ..default()
        }))
        .init_gizmo_group::<MyArrowGizmos>()
        .insert_resource(ClearColor(SKY_COLOR))
        // .init_gizmo_group::<MyGizmos>()
        .insert_resource(Time::<Fixed>::from_hz(30.0)) //This messes with time.
        .add_systems(Startup, (
            spawn_camera, 
            // spawn_player, 
            spawn_player_with_gun,
            spawn_asteroides,
            initialize_asteroide_veloccity.after(spawn_asteroides)
        ))
        .add_systems(Update, (
            accelerate_player, 
            rotate_player, 
            throttle_player_velocity,
            throttle_asteroid_velocity,
            throttle_bullet_velocity,
            fire_bullet, 
            conservation_of_linear_momentum, 
            conservation_of_angular_momentum,
            wrap_position,
            self_destruct_countdown,
            draw_arrow,
        ))
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}


fn wrap_position(
    windows: Single<&Window>,
    mut transforms: Query<&mut Transform, With<Object>>
) {
    // Get the primary window
    let window = windows.into_inner();
    let x_bound = window.width() / 2.0;
    let y_bound = window.height() / 2.0;
    for mut transform in transforms.iter_mut() {
        if transform.translation.x > x_bound {
            transform.translation.x = -x_bound;
        } else if transform.translation.x < -x_bound {
            transform.translation.x = x_bound;
        }
        if transform.translation.y > y_bound {
            transform.translation.y = -y_bound;
        } else if transform.translation.y < -y_bound {
            transform.translation.y = y_bound;
        }
    }
}


