use bevy::prelude::*;

use crate::{addition_functions::*, entities::asteroid::destory_asteroide};
mod movement;
mod entities;
mod utillity;
mod addition_functions;
mod camera;
const PLAYER_THRUSTER_STRENGTH: f32 = 50.0f32;
const PLAYER_SPEED_MODIFIER: f32 = 2.0f32;
const PLAYER_ROT_SPEED_MODIFIER: f32 = 1.0f32;
const ASTEROID_SPEED_MODIFIER: f32 = 25.0f32;
const BULLET_SPEED_MODIFIER: f32 = 25.0f32;

const GRAVITY_WELL_STRENGTH: f32 = 9.8f32;

const SPEED_OF_LIGHT: f32 = 1000.0f32;
const MAXIMUM_LINEAR_PLAYER_SPEED: f32 = SPEED_OF_LIGHT;
const MAXIMUM_LINEAR_ASTEROID_SPEED: f32 = SPEED_OF_LIGHT;
const MAXIMUM_LINEAR_BULLET_SPEED: f32 = SPEED_OF_LIGHT;

const MAXIMUM_LINEAR_STEP_LENGTH: f32 = 50.0f32;

const TRAJECTORY_LENGTH: usize = 100;
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
    add_player_thrusters(&mut app);
    #[cfg(debug_assertions)]
    add_gizmos(&mut app);
    app.add_observer(destory_asteroide);

    app.run();
}





