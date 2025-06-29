use bevy::prelude::*;

use crate::{camera::following_camera::{make_camera_follow, move_following_camera}, entities::player::PlayerTag};
    
pub struct MyCameraPlugin;
impl Plugin for MyCameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,    (
            spawn_camera, 
            make_camera_follow::<PlayerTag>.after(spawn_camera),
            apply_camera_zoom.after(spawn_camera)
        )).add_systems(Update, move_following_camera);
    }
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
pub fn apply_camera_zoom(
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    for mut projection in query.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = 2.0; // Zoomed out; try 0.5 to zoom in
        }
    }
}

pub mod following_camera{
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct FollowingCameraTag;

    pub fn make_camera_follow<T:Component>(mut commands: Commands, query: Single<Entity, (With<T>, Without<FollowingCameraTag>)>) {
        commands.entity(query.into_inner()).insert(FollowingCameraTag);
    }
    pub fn move_following_camera(
        cam_query: Single<&mut Transform, With<Camera2d>>,
        target_query: Single<&Transform, (With<FollowingCameraTag>, Without<Camera2d>)>
    ){
        let target_pos = target_query.into_inner();
        let mut cam =cam_query.into_inner();
        cam.translation = target_pos.translation;
    }
}
