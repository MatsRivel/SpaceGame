pub mod hitbox;
pub mod health;

use bevy::prelude::*;
use crate::destruction::{health::Health, hitbox::{HitBox, HitBoxTrait}};

#[derive(Component,Debug,Default,Clone,Copy)]
#[require(HitBox)]
pub struct Destructible;

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct Destroying;

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
    attacker_query: Query<&Transform, With<Destroying>>,
) {
    for (defending_entity, defending_transform, defending_hitbox, health_option) in defender_query.iter_mut() {
        for attacker_transform in attacker_query.iter() {
            let defender_position = defending_transform.translation.truncate();
            let attacker_position = attacker_transform.translation.truncate();
            if defending_hitbox.is_in_hit_box(&defender_position, &attacker_position) {
                if let Some(mut health) = health_option {
                    (*health).apply_damage(25.0); // TODO: Use proper damage instead.
                    if health.is_alive() {
                        break;
                    }
                }
                // Trigger the observer system to call the destroy_destructible function
                commands.trigger_targets(DestroySomething,defending_entity);
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