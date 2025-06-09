use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::entities::player::Player;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyArrowGizmos;
pub fn draw_arrow(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyArrowGizmos>,
    query: Single<&Transform, With<Player>>
) {
    let transform = query.into_inner();
    let origin = transform.translation;
    let rot = transform.rotation;
    let forward = origin + rot * Vec3::Y;
    let position = origin + forward.normalize() * 25.0;
    gizmos
        .arrow_2d(
            origin.truncate(),
            position.truncate(),
            GREEN,
        )
        .with_tip_length(10.);
    }
