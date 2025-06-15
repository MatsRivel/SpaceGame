use bevy::color::palettes::css::{BLUE, DARK_RED, GREEN, INDIAN_RED, ORANGE, RED, WHITE, WHITE_SMOKE, YELLOW};
use bevy::prelude::*;
use crate::movement::gravity::gravity_2d::{GravityProducer, Mass, EVENT_HORIZON, HIGH_GRAVITY, LOW_GRAVITY, NO_GRAVITY};
use crate::movement::velocity::linear_velocity::Velocity;
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
pub fn draw_player_trajectory(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyArrowGizmos>,
    player_query: Single<(&Transform, &Velocity, &Mass), With<PlayerTag>>,
    gravity_query: Single<(&Transform, &Mass, &GravityProducer), Without<PlayerTag>>){
    let (player_transform, player_velocity, player_mass) = player_query.into_inner();
    let player_position = player_transform.translation.truncate().clone();
    let player_speed = **player_velocity;
    let (well_position, well_mass, gravity_force) = gravity_query.into_inner();
    let n = 300;
    let force = **well_mass * **player_mass * **gravity_force; 
    let time_step = 0.1;
    let temp = calculate_trajectory(player_position, player_speed, well_positionforce, time_step );
    // TODO: Continue drawing future positions
}
fn calculate_trajectory(
    mut position: Vec2,
    mut velocity: Vec2,
    gravity_source: Vec2,
    gm: f32, // G * M
    dt: f32,
    steps: usize,
) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(steps);

    for _ in 0..steps {
        let direction = gravity_source - position;
        let distance_squared = direction.length_squared();

        // Avoid division by zero or extreme acceleration
        if distance_squared > 1e-6 {
            let distance = distance_squared.sqrt();
            let acceleration = direction.normalize() * (gm / distance_squared);

            // Euler integration step
            velocity += acceleration * dt;
            position += velocity * dt;
        }

        points.push(position);
    }

    points
}