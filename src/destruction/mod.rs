pub mod hitbox;
pub mod health;

use std::{ops::SubAssign, time::Duration};

use bevy::prelude::*;
use crate::destruction::{health::Health, hitbox::{HitBox, HitBoxTrait}};

#[derive(Component,Debug,Default,Clone,Copy)]
#[require(HitBox)]
pub struct Destructible;

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct Destroying;

#[derive(Component,Debug,Clone,Copy)]
#[require(Destroying)]
pub struct MultiHitDestroying(i32);
impl MultiHitDestroying{
    pub fn has_more_hits(&self)->bool{
        self.0 > 0
    }
    pub fn new(n_hits:i32)->Self{
        Self(n_hits)
    }
}
impl Default for MultiHitDestroying{
    fn default() -> Self {
        Self(2)
    }
}
impl SubAssign<i32> for MultiHitDestroying{
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs
    }
}
#[derive(Component,Debug,Default,Clone)]
#[require(Destroying)]
pub struct DestroyingHitTimer{
    hit_delay: Duration, // Todo: maybe don't need this, as timer already holds it?
    timer: Timer
}
impl DestroyingHitTimer{
    pub fn new(hit_delay_millis: u64)->Self{
        let duration = Duration::from_millis(hit_delay_millis);
        Self { hit_delay: duration, timer: Timer::new(duration, TimerMode::Once) }
    }
}

#[derive(Event)]
pub struct DestroySomething;

pub fn destroy_destructible(trigger: Trigger<DestroySomething>, mut commands: Commands) {
    let id = trigger.target();
    if let Ok(mut entity) = commands.get_entity(id) {
        entity.despawn();
    }
}

pub fn check_for_destruction(
    mut commands: Commands,
    mut defender_query: Query<(Entity, &Transform, &HitBox, Option<&mut Health>), With<Destructible>>,
    mut attacker_query: Query<(Entity,&Transform, Option<&mut DestroyingHitTimer>, Option<&mut MultiHitDestroying>), With<Destroying>>,
) {
    for (defending_entity, defending_transform, defending_hitbox, health_option) in defender_query.iter_mut() {
        for (attacking_entity,attacker_transform, attacker_hit_timer, multi_hit_attack) in attacker_query.iter_mut() {
            let defender_position = defending_transform.translation.truncate();
            let attacker_position = attacker_transform.translation.truncate();
            if defending_hitbox.is_in_hit_box(&defender_position, &attacker_position) {
                let mut should_be_destroyed = true;
                let mut should_damage = true;
                let mut should_destroy = true;
                // If there is an attack timer, we should only let it damage stuff once each timer round.
                if let Some(mut timer) = attacker_hit_timer{
                    if timer.timer.finished(){
                        let duration = timer.hit_delay;
                        (timer).timer.set_duration(duration);
                        (timer).timer.reset();
                    }else{
                        should_damage = false;
                    }
                }

                if should_damage && let Some(mut health) = health_option {
                    dbg!("Decrementing health");
                    (*health).apply_damage(1.0); // TODO: Use proper damage instead.
                    if health.is_alive() {
                        should_destroy = false;
                    }
                }
                if let Some(mut hit_count) = multi_hit_attack{
                    if should_damage{
                        *hit_count -= 1;
                    }
                    if hit_count.has_more_hits(){
                        should_be_destroyed = false;
                    }
                }

                // Once a bullet hits something, we remove its abillity to hit more stuff.
                if should_be_destroyed && let Ok(mut attacking_entity_commands) = commands.get_entity(attacking_entity){
                    attacking_entity_commands.despawn();
                    // let _ = attacking_entity_commands.try_remove::<Destroying>();
                    // let _ = attacking_entity_commands.try_remove::<Sprite>();
                }

                if should_destroy{
                    dbg!("Destroying health");
                    // Trigger the observer system to call the destroy_destructible function
                    commands.trigger_targets(DestroySomething,defending_entity);
                }
                break; // avoid duplicate triggers
            }
        }
    }
}
pub struct DestructionPlugin;
impl Plugin for DestructionPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_destruction)
            .add_observer(destroy_destructible); // Global Observer. Triggers for any event.

    }
}