use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SelfDestructTimer{
    timer: Timer
}
impl SelfDestructTimer{
    pub fn new(seconds: f32)->Self{
        Self { timer: Timer::from_seconds(seconds, TimerMode::Once) }
    }
}

pub fn self_destruct_countdown(mut commands: Commands, time: Res<Time>, mut timer_query: Query<(Entity, &mut SelfDestructTimer)>){
    for (entity, mut timer) in timer_query.iter_mut(){
        timer.timer.tick(time.delta());
        if timer.timer.finished(){
            commands.entity(entity).despawn();
        }
    }
}