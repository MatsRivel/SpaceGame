use bevy::color::palettes::css::{DARK_RED, GREEN, INDIAN_RED, ORANGE, RED, WHITE, YELLOW};
use bevy::prelude::*;
use crate::movement::gravity::gravity_2d::{GravityProducer, EVENT_HORIZON, HIGH_GRAVITY, LOW_GRAVITY, NO_GRAVITY};
use crate::utillity::forward::ForwardUnit;
use crate::entities::player::PlayerTag;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyArrowGizmos;
pub fn draw_arrow(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyArrowGizmos>,
    query: Single<&Transform, With<PlayerTag>>
) {
    let transform = query.into_inner();
    let origin = transform.translation.truncate();
    let forward = transform.forward_unit_vector().truncate();
    let position = origin + forward* 25.0;
    gizmos
        .arrow_2d(
            origin,
            position,
            GREEN,
        )
        .with_tip_length(10.);
}

pub fn to_well(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyArrowGizmos>,
    query: Single<&Transform, With<PlayerTag>>,
    well_query: Query<&Transform, With<GravityProducer>>
) {
    let transform = query.into_inner();
    let origin = transform.translation.truncate();
    for position in well_query.iter().map(|well| well.translation.truncate()){
        gizmos
            .arrow_2d(
                origin,
                position,
                GREEN,
            )
            .with_tip_length(10.);
    }
}

pub fn draw_gravity_falloff(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyArrowGizmos>,
    query: Query<&Transform, With<GravityProducer>>,
) {
    for position in query.iter().map(|well| well.translation.truncate()){
        gizmos
            .circle_2d(
                position,
                EVENT_HORIZON,
                RED
            );
        gizmos
            .circle_2d(
                position,
                HIGH_GRAVITY,
                ORANGE
            );
        gizmos
            .circle_2d(
                position,
                LOW_GRAVITY,
                YELLOW
            );
        gizmos
            .circle_2d(
                position,
                NO_GRAVITY,
                WHITE
            );
    }
}