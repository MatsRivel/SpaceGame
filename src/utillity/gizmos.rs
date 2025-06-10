use bevy::{color::palettes::css::GREEN, prelude::*};
use crate::utillity::forward::ForwardUnit;
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
    let forward = transform.forward_unit_vector();
    let position = origin + forward* 25.0;
    gizmos
        .arrow_2d(
            origin.truncate(),
            position.truncate(),
            GREEN,
        )
        .with_tip_length(10.);
    }
