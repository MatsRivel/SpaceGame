use bevy::prelude::*;
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyGizmos;
fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vk");
    }
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // primary_window: Some(Window {
            //     resolution: WindowResolution::new(200., 100.).with_scale_factor_override(1.0),
            //     ..Default::default()
            // }),
            ..default()
        }))
        .insert_resource(ClearColor(SKY_COLOR))
        .init_gizmo_group::<MyGizmos>()
        .insert_resource(Time::<Fixed>::from_hz(30.0)) //This messes with time.
        .add_systems(Startup, (spawn_camera,spawn_player))
        
        // .add_systems(FixedUpdate, ...)
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = "assets/sprites/Ships/ship-a/shipa1.png";
    let image = asset_server.load(asset_path);
    commands.spawn(Sprite::from_image(image));
}
