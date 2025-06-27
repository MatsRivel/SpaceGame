use bevy::color::palettes::css::{GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW};
use bevy::prelude::*;
use crate::entities::player::PlayerTag;
use crate::gravity::gravity_2d::{GravityProducer, Mass, EVENT_HORIZON_DISTANCE, HIGH_GRAVITY_DISTANCE, LOW_GRAVITY_DISTANCE, NO_GRAVITY_DISTANCE};
use crate::gravity::gravity_plugin::GRAVITY_FUNC;
use crate::movement::velocity::linear_velocity::Velocity;
use crate::utillity::forward::ForwardUnit;
use crate::TRAJECTORY_LENGTH;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyArrowGizmos;
pub fn draw_arrow(
    mut gizmos: Gizmos,
    _: Gizmos<MyArrowGizmos>,
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
    _: Gizmos<MyArrowGizmos>,
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
    _: Gizmos<MyArrowGizmos>,
    query: Query<&Transform, With<GravityProducer>>,
) {
    for position in query.iter().map(|well| well.translation.truncate()){
        gizmos
            .circle_2d(
                position,
                EVENT_HORIZON_DISTANCE,
                RED
            );
        gizmos
            .circle_2d(
                position,
                HIGH_GRAVITY_DISTANCE,
                ORANGE
            );
        gizmos
            .circle_2d(
                position,
                LOW_GRAVITY_DISTANCE,
                YELLOW
            );
        gizmos
            .circle_2d(
                position,
                NO_GRAVITY_DISTANCE,
                WHITE
            );
    }
}
pub fn draw_player_trajectory<const N: usize>(
    time: Res<Time>,
    mut gizmos: Gizmos,
    _: Gizmos<MyArrowGizmos>,
    gravity_query: Query<(&Transform, &GravityProducer, &Mass)>,
    player_query: Single<(&Transform, &Mass, &Velocity), With<PlayerTag>>
) {
    // Collect producer data first, so we don't hold the borrow
    let producers: Vec<(Vec2,f32,f32)> = gravity_query
        .iter()
        .map(|(transform, producer, mass)| (transform.translation.truncate(), **producer, **mass)).collect();

    let (player_transform, player_mass,player_velocity) = player_query.into_inner();
    let player_pos = player_transform.translation.truncate();
    let player_mass = **player_mass;
    let player_velocity = **player_velocity;
    let (mut pos, mass, mut velocity) = (player_pos, player_mass, player_velocity);
    let mut to = pos;
    for _ in 0..N{
        for (producer_transform, force, producer_mass) in producers.iter() {
            let from = to;
            velocity += GRAVITY_FUNC(producer_transform, *force, *producer_mass, &pos, mass, time.delta_secs()+1.0);
            pos += velocity;
            to = pos;
            gizmos.line_2d(from, to, PURPLE);
        }
    }
}

pub struct GizmoPlugins;
impl Plugin for GizmoPlugins{
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MyArrowGizmos>()
            .add_systems(Update, (
                to_well,
                draw_player_trajectory::<TRAJECTORY_LENGTH>,
                draw_arrow,
                draw_gravity_falloff,
        ));
    }
}