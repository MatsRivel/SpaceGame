use bevy::prelude::*;
use mats_game_lib::camera::mycamera::MyCameraPlugin;
use mats_game_lib::entities::player::{PlayerTag,PlayerPlugin};

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
        MyCameraPlugin::<PlayerTag>(std::marker::PhantomData::<PlayerTag>),
        PlayerPlugin
    ));

    app.run();
}





