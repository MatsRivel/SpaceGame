pub mod gravity_2d{
    use std::ops::Deref;
    use bevy::prelude::*;

    use crate::{entities::black_hole::GravityWell, movement::velocity::linear_velocity::Velocity};
    pub const EVENT_HORIZON: f32 = 10.0;
    pub const HIGH_GRAVITY: f32 = 250.0;
    pub const LOW_GRAVITY: f32 = 500.0;
    pub const NO_GRAVITY: f32 = 1000.0;
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
        Self { mass: 100.0 }
    }
    }
    
    #[derive(Component,Default)]
    #[require(Mass, Velocity)]
    pub struct GravityAffected;
    
    #[derive(Component, Default, Clone, Copy)]
    #[require(Mass)]
    pub struct GravityProducer { force: f32 }
    impl GravityProducer{
        pub fn new(force: f32) -> Self {
            Self { force }
        }
    }
    impl Deref for GravityProducer{
        type Target = f32;
    
        fn deref(&self) -> &Self::Target {
            &self.force
        }
    }
    pub fn apply_gravity(
        time: Res<Time>,
        mut param_set: ParamSet<(
            Query<(&Transform, &GravityProducer, &Mass)>,
            Query<(&Transform, &Mass, &mut Velocity), With<GravityAffected>>,
        )>,
    ) {
        // Collect producer data first, so we don't hold the borrow
        let producers: Vec<(Vec3, f32, f32)> = {
            let producer_query = param_set.p0();
            producer_query
                .iter()
                .map(|(transform, producer, mass)| (transform.translation, **producer, **mass))
                .collect()
        };

        let mut affected_query = param_set.p1();
        for (affected_translation, affected_mass,mut velocity) in affected_query.iter_mut().map(|(transform, mass, velocity)|(transform.translation, **mass, velocity)){
            for (producer_transform, force, producer_mass) in &producers {
                let dist = affected_translation.distance(*producer_transform);
                let delta = producer_transform - affected_translation;
                if delta == Vec3::ZERO{
                    continue;
                }

                let force_multiplier = match dist{
                    ..EVENT_HORIZON => 10.0,
                    EVENT_HORIZON..HIGH_GRAVITY => 3.0,
                    HIGH_GRAVITY..LOW_GRAVITY => 2.0,
                    LOW_GRAVITY..NO_GRAVITY => 1.0,
                    NO_GRAVITY.. => continue,
                    _ => continue
                };
                let direction = delta.normalize();
                let relative_force = force_multiplier * force* affected_mass *producer_mass / dist;
                let delta_time_force = relative_force * time.delta_secs();
                let applied_vec = direction * delta_time_force;
                *velocity += applied_vec.truncate();

            }
        }
    }
    #[derive(Component)]
    pub struct InsideEventHorizon;
    #[derive(Event,Debug)]
    pub struct EnteredEventHorizon(Entity);

    pub fn event_horizon_entry_event(
        mut commands: Commands,
        gravity_well_query: Query<&Transform, With<GravityWell>>,
        target_query: Query<(Entity, &Transform), (With<GravityAffected>, Without<InsideEventHorizon>)>,
        mut event_writer: EventWriter<EnteredEventHorizon>
    ){
        for gw_pos in gravity_well_query.iter().map(|v| v.translation.truncate()){
            for (entity,target_pos) in target_query.iter().map(|(e,v)| (e, v.translation.truncate())){
                let distance = target_pos.distance(gw_pos);
                if distance < EVENT_HORIZON{
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
