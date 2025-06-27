use bevy::prelude::*;
#[derive(Component)]
pub struct Health{
    max_health: f32,
    current_health: f32
}
impl Health{
    pub fn new(health:f32)->Self{
        Self { max_health: health, current_health: health }
    }
    #[allow(unused)]
    pub fn new_injured(max_health:f32,current_health:f32)->Self{
        Self { max_health, current_health }
    }
    fn change_health(&mut self, health_change: f32){
        self.current_health =  (self.current_health+health_change).max(0.0).min(self.max_health)
    }
    pub fn apply_damage(&mut self, damage: f32){
        self.change_health(-damage);
    }
    #[allow(unused)]
    pub fn apply_healing(&mut self, healing: f32){
        self.change_health(healing);
    }
    pub fn is_alive(&self)->bool{
        self.current_health != 0.0
    }
}
impl Default for Health{
    fn default() -> Self {
        Self::new(100.0)
    }
}