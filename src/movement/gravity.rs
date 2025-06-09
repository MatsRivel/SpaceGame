pub mod gravity_2d{
    use bevy::prelude::*;
    #[derive(Component,Default)]
    pub struct GravityAffected;
    
    #[derive(Component,Default)]
    pub struct GravityProducer { force: f32 }

}