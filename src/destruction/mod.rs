use bevy::prelude::*;


#[derive(Component,Debug,Default,Clone,Copy)]
pub struct Destructible;

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct Destroying;

#[derive(Event)]
pub struct DestroySomething;

pub fn destroy_destructible(trigger: Trigger<DestroySomething>, mut commands: Commands) {
    dbg!("'destroy_destructible' was triggered!");
    let id = trigger.target();
    if let Ok(mut entity) = commands.get_entity(id) {
        entity.despawn();
        dbg!("Something was destroyed!");
    }
}
pub fn destroy_asteroid(trigger: Trigger<DestroySomething>, mut commands: Commands) {
    dbg!("'destroy_asteroid' was triggered!");
    let id = trigger.target();
    if let Ok(mut entity) = commands.get_entity(id) {
        entity.despawn();
        dbg!("Asteroid was destroyed!");
    }
}


pub fn check_for_destruction(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform), With<Destructible>>,
    bullet_query: Query<&Transform, With<Destroying>>,
) {
    let destruction_distance = 50.0;

    for (asteroid_entity, asteroid_transform) in asteroid_query.iter() {
        for bullet_transform in bullet_query.iter() {
            let distance = asteroid_transform
                .translation
                .truncate()
                .distance(bullet_transform.translation.truncate());

            if distance < destruction_distance {
                // Trigger the observer system to call the destroy_asteroid function
                commands.trigger_targets(DestroySomething,asteroid_entity);
                break; // avoid duplicate triggers
            }
        }
    }
}