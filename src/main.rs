use bevy::prelude::*;

use crate::camera::MyCameraPlugin;
use crate::entities::gravity_well::GravityWellPlugin;
use crate::entities::player::PlayerPlugin;
use crate::movement::linear_movement_2d::LinearMovement2DPlugin;
use crate::thrusters::thrusters_plugin::ThrusterPlugin;
use crate::gun::{gun_plugin::GunPlugin};
use crate::gravity::{gravity_plugin::GravityPlugin};
use crate::entities::asteroid::AsteroidPlugin;
use crate::destruction::destroy_destructible;
use crate::utillity::gizmos::GizmoPlugins;
use crate::utillity::wrap_map::WorldWrapPlugin;

mod movement;
mod entities;
mod utillity;
mod camera;
mod thrusters;
mod gun;
mod bullet;
mod gravity;
mod destruction;

const WORLD_HEIGHT: f32 = 20_000.0f32;
const WORLD_WIDTH: f32 = 20_000.0f32;

const PLAYER_THRUSTER_STRENGTH: f32 = 200.0f32;
const PLAYER_SPEED_MODIFIER: f32 = 2.0f32;
const PLAYER_ROT_SPEED_MODIFIER: f32 = 1.0f32;
const ASTEROID_SPEED_MODIFIER: f32 = 10.0f32;
const BULLET_SPEED_MODIFIER: f32 = 25.0f32;

const GRAVITY_WELL_STRENGTH: f32 = 9.8f32;

const SPEED_OF_LIGHT: f32 = 1000.0f32;
const MAXIMUM_LINEAR_PLAYER_SPEED: f32 = SPEED_OF_LIGHT;
const MAXIMUM_LINEAR_ASTEROID_SPEED: f32 = SPEED_OF_LIGHT;
const MAXIMUM_LINEAR_BULLET_SPEED: f32 = SPEED_OF_LIGHT;

const MAXIMUM_LINEAR_STEP_LENGTH: f32 = 50.0f32;

const TRAJECTORY_LENGTH: usize = 100;
const PLAYER_BULLET_IMAGE_PATH: &str = r"sprites\FX\bullet\bullet1.png";
const PLAYER_BODY_IMAGE_PATH: &str = r"AI_Assets\Body\Space_sattelite_core...-1670999352-0 (1).png";
const PLAYER_GUN_IMAGE_PATH: &str = r"AI_Assets\Weapon\Space_station_weapon...-1138415846-0 (3).png";
fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vk");
    }
    const SKY_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.5);
    let mut app = App::new();
    app.insert_resource(ClearColor(SKY_COLOR));
    app.insert_resource(Time::<Fixed>::from_hz(30.0)) ;//This messe s with time.
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        // primary_window: Some(Window {
        //     resolution: WindowResolution::new(200., 100.).with_scale_factor_override(1.0),
        //     ..Default::default()
        // }),
        ..default()
    }));
    app.add_plugins((
        GunPlugin,
        GravityPlugin,
        ThrusterPlugin,
        WorldWrapPlugin,
        AsteroidPlugin,
        MyCameraPlugin,
        LinearMovement2DPlugin,
        GravityWellPlugin,
        PlayerPlugin
    ));
    app.add_observer(destroy_destructible); // Global Observer. Triggers for any event.

    #[cfg(debug_assertions)]
    {
        app.add_plugins(GizmoPlugins);
    }

    app.run();
}





