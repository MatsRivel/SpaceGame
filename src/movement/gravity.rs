pub mod gravity_2d{
    use std::ops::Deref;

    use bevy::prelude::*;
    #[derive(Component,Default)]
    pub struct GravityAffected;
    
    #[derive(Component, Default, Clone, Copy)]
    pub struct GravityProducer { force: f32 }
    impl Deref for GravityProducer{
        type Target = f32;
    
        fn deref(&self) -> &Self::Target {
            &self.force
        }
    }
    pub fn apply_gravity(time: Res<Time>, producer_query: Query<(&Transform, &GravityProducer)>, mut affected_query: Query<&mut Transform, With<GravityAffected>>){
        for (producer, force) in producer_query.iter(){
            for mut affected in affected_query.iter_mut(){
                affected.translation = affected.translation.move_towards(producer.translation, **force * time.delta_secs());
            }
        }
    }

}
