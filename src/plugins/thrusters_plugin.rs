use bevy::prelude::*;

use crate::{entities::{player::{spawn_player, PlayerTag}, thrusters::{HasThrusters, Thrusters}}, PLAYER_THRUSTER_STRENGTH};
pub struct ThrusterPlugin;
impl Plugin for ThrusterPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, 
            give_t_thrusters::<PlayerTag>.after(spawn_player)
        ).add_systems(Update, show_thrust_fire::<Thrusters>);
    }
}
pub fn give_t_thrusters<T:Component>(mut commands: Commands, asset_server: Res<AssetServer>, query: Single<Entity, (With<T>,Without<Thrusters>)>){
    let asset_path = r"sprites\Ships\ship-b\ship-b1.png";
    let image = asset_server.load(asset_path);
    commands
        .entity(query.into_inner())
        .insert((
            HasThrusters, 
        )).with_child(  (
            Thrusters(PLAYER_THRUSTER_STRENGTH),
            Sprite::from_image(image),
            Transform::from_translation(Vec2::new(0.0, -25.0).extend(0.0))
        ));
}
pub fn show_thrust_fire<T:Component>(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<&mut Sprite, With<T>>, // Note to self: Should edit child, not player
    asset_server: Res<AssetServer>
){
    let thrusters_active = keyboard_input.just_pressed(KeyCode::KeyW);
    let thrusters_inactive = keyboard_input.just_released(KeyCode::KeyW);
    let asset_path = match (thrusters_active, thrusters_inactive){
        (true, false) => r"sprites\Ships\ship-b\ship-b2.png",
        (false, true) => r"sprites\Ships\ship-b\ship-b1.png",
        _ => return
    };
    let image = asset_server.load(asset_path);
    let mut temp = query.into_inner();
    *temp = image.into();

}