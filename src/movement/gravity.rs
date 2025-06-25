pub mod gravity_2d{
    use std::ops::Deref;
    use bevy::prelude::*;

    use crate::{entities::gravity_well::GravityWell, movement::velocity::{linear_acceleration::LinearAcceleration}};
    pub const EVENT_HORIZON_DISTANCE: f32 = 1000.0;
    pub const SAVIOUR_ZONE_DISTANCE: f32 = 3500.0;
    pub const HIGH_GRAVITY_DISTANCE: f32 = 3000.0;
    pub const LOW_GRAVITY_DISTANCE: f32 = 4000.0;
    pub const NO_GRAVITY_DISTANCE: f32 = 5000.0;

    pub const EVENT_HORIZON_STRENGTH: f32 = 0.0;
    pub const SAVIOUR_ZONE_STRENGTH: f32 = 10.0;
    pub const HIGH_GRAVITY_STRENGTH: f32 = 2.0;
    pub const LOW_GRAVITY_STRENGTH: f32 = 1.0;
    pub const TRIVIAL_GRAVITY_STRENGTH: f32 = 0.5;
    
    pub const TRUE_GRAVITY_DISTANCE: f32 = NO_GRAVITY_DISTANCE;
    // pub const TRUE_GRAVITY_STRENGTH: f32 = 250.0;
    #[derive(Component)]
    pub struct Mass{mass:f32}
    
    impl Mass {
        pub fn new(mass:f32)->Self{
            Self{mass}
        }
    }
    impl Deref for Mass{
        type Target = f32;
    
        fn deref(&self) -> &Self::Target {
            &self.mass
        }
    }
    impl Default for Mass{
        fn default() -> Self {
        Self { mass: 10_000.0 }
    }
    }
    
    #[derive(Component,Default)]
    #[require(Mass, LinearAcceleration)]
    pub struct GravityAffected;
    
    #[derive(Component, Default, Clone, Copy)]
    #[require(Mass)]
    pub struct GravityProducer { force: f32 }
    impl GravityProducer{
        pub fn new(force: f32) -> Self {
            Self { force }
        }
        #[allow(unused)]
        pub fn force_multiplier_simplified(dist:f32)->Option<f32>{
            match dist{
                ..EVENT_HORIZON_DISTANCE => Some(EVENT_HORIZON_STRENGTH),
                EVENT_HORIZON_DISTANCE..HIGH_GRAVITY_DISTANCE => Some(HIGH_GRAVITY_STRENGTH),
                HIGH_GRAVITY_DISTANCE..LOW_GRAVITY_DISTANCE => Some(LOW_GRAVITY_STRENGTH),
                LOW_GRAVITY_DISTANCE..NO_GRAVITY_DISTANCE => Some(TRIVIAL_GRAVITY_STRENGTH),
                _ => None,
            }
        }

        pub fn saviour_movement(dist:f32, direction: Vec2)->Option<Vec2>{
            match dist{
                EVENT_HORIZON_DISTANCE..SAVIOUR_ZONE_DISTANCE => Some(Vec2::new(-direction.y, direction.x)*SAVIOUR_ZONE_STRENGTH),
                _ => None
            }
        }
    }
    impl Deref for GravityProducer{
        type Target = f32;
    
        fn deref(&self) -> &Self::Target {
            &self.force
        }
    }
    #[allow(clippy::type_complexity)] // Does not make sense to make type for this query.
    pub fn build_gravity_function<F>(
        gravity_function: F,
    ) -> impl Fn(
        Res<Time>,
        ParamSet<(
            Query<(&Transform, &GravityProducer, &Mass)>,
            Query<(&Transform, &Mass, &mut LinearAcceleration), With<GravityAffected>>,
        )>,
    ) + Clone
    where
        F: Fn(&Vec2, f32, f32, &Vec2, f32, f32)->Vec2 + Clone + 'static,
    {
        move |time: Res<Time>, mut param_set: ParamSet<(
            Query<(&Transform, &GravityProducer, &Mass)>,
            Query<(&Transform, &Mass, &mut LinearAcceleration), With<GravityAffected>>,
        )>| {
            // Collect producer data first, so we don't hold the borrow
            let producers = {
                let producer_query = param_set.p0();
                producer_query
                    .iter()
                    .map(|(transform, producer, mass)| (transform.translation.truncate(), **producer, **mass))
                    .collect::<Vec<(Vec2, f32, f32)>>()
            };

            let mut affected_query = param_set.p1();
            for (pos, mass, mut acceleration) in affected_query.iter_mut() {
                let player_pos = pos.translation.truncate();
                let player_mass = **mass;
                for (producer_transform, force, producer_mass) in producers.iter() {
                    *acceleration += gravity_function(producer_transform, *force, *producer_mass, &player_pos, player_mass, time.delta_secs());
                }
            }
        }
    }
    #[allow(unused)]
    // Descrete psudo-gravity.
    pub fn gravity_calculation_simplified(producer_transform: &Vec2, force: f32, producer_mass: f32, player_pos: &Vec2, player_mass: f32, delta_time: f32)->Vec2{
        let dist = player_pos.distance(*producer_transform);
        let delta = producer_transform - player_pos;
        if delta != Vec2::ZERO && let Some(force_multiplier) = GravityProducer::force_multiplier_simplified(dist){
            let direction = delta.normalize();
            let relative_force = force_multiplier * force* player_mass *producer_mass / dist;
            let delta_time_force = relative_force * delta_time * 0.5;
            let saviour_vec = GravityProducer::saviour_movement(dist,direction).unwrap_or(Vec2::ZERO);
            let applied_vec = (direction + saviour_vec)* delta_time_force ;
            return applied_vec;
        }
        Vec2::ZERO
    }
    #[allow(unused)]
    // 2D but with real gravity. Falls off much faster.
    pub fn gravity_calculation_true(producer_transform: &Vec2, force: f32, producer_mass: f32, player_pos: &Vec2, player_mass: f32, delta_time: f32)->Vec2{
        let dist = player_pos.distance(*producer_transform);
        let delta = producer_transform - player_pos;
        if delta != Vec2::ZERO && dist < TRUE_GRAVITY_DISTANCE{
            let direction = delta.normalize();
            let relative_force = force* player_mass *producer_mass / dist.powf(2.0);
            let delta_time_force = relative_force * delta_time * 0.5;
            let applied_vec = direction* delta_time_force;
            return applied_vec;
        }
        Vec2::ZERO
    }
    /// 2D version of gravity. Stronger at a distance, same nearby. Falls off slower.
    pub fn gravity_calculation_flat_true(producer_transform: &Vec2, force: f32, producer_mass: f32, player_pos: &Vec2, player_mass: f32, delta_time: f32)->Vec2{
        let dist = player_pos.distance(*producer_transform);
        let delta = producer_transform - player_pos;
        if delta != Vec2::ZERO && dist < TRUE_GRAVITY_DISTANCE{
            let direction = delta.normalize();
            let relative_force = force* player_mass *producer_mass / dist;
            let delta_time_force = relative_force * delta_time * 0.5;
            let applied_vec = direction* delta_time_force;
            return applied_vec;
        }
        Vec2::ZERO
    }
    #[allow(unused)]
    /// 2D version of gravity. Stronger at a distance, same nearby. Falls off slower.
    pub fn gravity_calculation_flat_saviour(producer_transform: &Vec2, force: f32, producer_mass: f32, player_pos: &Vec2, player_mass: f32, delta_time: f32)->Vec2{
        let dist = player_pos.distance(*producer_transform);
        let delta = producer_transform - player_pos;
        if delta != Vec2::ZERO && dist < TRUE_GRAVITY_DISTANCE{
            let direction = delta.normalize();
            let saviour_vec = GravityProducer::saviour_movement(dist,direction).unwrap_or(Vec2::ZERO);
            let relative_force = force* player_mass *producer_mass / dist;
            let delta_time_force = relative_force * delta_time * 0.5;
            let applied_vec = (direction + saviour_vec)* delta_time_force;
            
            return applied_vec;
        }
        Vec2::ZERO
    }

    #[derive(Component)]
    pub struct InsideEventHorizon;
    #[derive(Event,Debug)]
    pub struct EnteredEventHorizon(Entity);

    #[allow(clippy::type_complexity)] // Does not make sense to make type for this query.
    pub fn event_horizon_entry_event(
        mut commands: Commands,
        gravity_well_query: Query<&Transform, With<GravityWell>>,
        target_query: Query<(Entity, &Transform), (With<GravityAffected>, Without<InsideEventHorizon>)>,
        mut event_writer: EventWriter<EnteredEventHorizon>
    ){
        for gw_pos in gravity_well_query.iter().map(|v| v.translation.truncate()){
            for (entity,target_pos) in target_query.iter().map(|(e,v)| (e, v.translation.truncate())){
                let distance = target_pos.distance(gw_pos);
                if distance < EVENT_HORIZON_DISTANCE{
                    event_writer.write(EnteredEventHorizon(entity));
                    commands.entity(entity).insert(InsideEventHorizon);
                }
            }
        }
    }
    pub fn crush_when_inside_event_horizon(
    mut events: EventReader<EnteredEventHorizon>,
    mut commands: Commands
    ){
        for entity in events.read(){
            commands.entity(entity.0).despawn();
        }
    }
}
