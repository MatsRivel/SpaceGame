use bevy::prelude::*;

use crate::entities::player::PlayerTag;
use crate::movement::gravity::gravity_2d::GravityAffected;
use crate::entities::object::Object;
use crate::movement::velocity::linear_velocity::Velocity;
use crate::utillity::timing::SelfDestructTimer;
use crate::{BULLET_SPEED_MODIFIER};


#[derive(Component, Default)]
pub struct HasGunTag;

#[derive(Component)]
#[require(HasGunTag)]
pub struct Gun{
    pub attack_speed: f32,
    pub magazine_size: u32,
    pub reload_speed: u32,
    pub bullet_factory: BulletFactory,

}
impl Gun{
    pub fn new(attack_speed: f32, magazine_size: u32, reload_speed: u32, bullet_image: Option<Handle<Image>>) -> Self{
        Self { attack_speed, magazine_size, reload_speed, bullet_factory: BulletFactory {bullet_image, ..Default::default() }}
    }
}
impl MakeBullet for Gun{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_transform: &Transform) -> (Bullet, Velocity, bevy::prelude::Transform, bevy::prelude::Sprite, SelfDestructTimer) {
        self.bullet_factory.make_bullet_bundle(possible_velocity, parent_transform)
    }
}

impl Default for Gun{
    fn default() -> Self {
        Self { 
            attack_speed: 1.0, 
            magazine_size: 5,
            reload_speed: 5,
            bullet_factory: BulletFactory::default(),
            
        }
    }
}
pub struct BulletFactory{
    pub damage: f32,
    pub radius: f32,
    pub mass: f32,
    pub lifetime: f32,
    pub speed: f32,
    bullet_image: Option<Handle<Image>>
    
}
impl MakeBullet for BulletFactory{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_transform: &Transform)->(Bullet, Velocity, bevy::prelude::Transform, bevy::prelude::Sprite, SelfDestructTimer){
        let bullet = Bullet::new(self.damage, self.radius, self.mass);
        let parent_velocity = **possible_velocity.unwrap_or(&Velocity::new(Vec2::ONE));
        let rotation_adjusted_movement = parent_transform.rotation.mul_vec3(Vec3::Y).truncate()*self.speed;
        let velocity = Velocity::new(parent_velocity + rotation_adjusted_movement);
        let sprite = {
            if let Some(bullet_image) = &self.bullet_image{
                Sprite::from_image(bullet_image.clone())
            }else{
                Sprite::default()
            }
        };
        (
            bullet,
            velocity,
            *parent_transform,
            sprite,
            SelfDestructTimer::new(self.lifetime)
        )
    }
}
impl Default for BulletFactory{
    fn default() -> Self {
        Self {
            damage: 1.0, 
            radius: 1.0, 
            mass: 1.0,
            lifetime: 6.0,
            speed: BULLET_SPEED_MODIFIER,
            bullet_image: None
        }
    }
}

pub trait MakeBullet{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_transform: &Transform)->(Bullet, Velocity, bevy::prelude::Transform, bevy::prelude::Sprite, SelfDestructTimer);
}


#[derive(Component)]
#[require(Object, GravityAffected, SelfDestructTimer)]
pub struct Bullet{
    pub damage: f32,
    pub radius: f32,
    pub mass: f32,
}
impl Bullet{
    pub fn new(damage: f32,radius: f32,mass: f32) -> Self{
        Self { damage, radius, mass}
    }
}
pub fn fire_bullet(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    gun_query: Query<&Gun>,
    wielder_query: Query<(&Transform, &Children, Option<&Velocity>, Option<&PlayerTag>), With<HasGunTag>>){
    if !keyboard_input.just_pressed(KeyCode::Space) { return;}
    for (transform, children, possible_velocity, possible_player_tag) in wielder_query{
        if possible_player_tag.is_some() && !keyboard_input.just_pressed(KeyCode::Space){
            // Players only fire when space is pressed.
            continue; 
        }else if possible_player_tag.is_none(){
            panic!("\n --- Not Implemented! ---\nWe haven't considered other entities shooting yet...\n")
        }
        for child in children.iter() {
            if let Ok(gun) = gun_query.get(child) {
                let bullet_bundle = gun.make_bullet_bundle(possible_velocity, transform);
                commands.spawn(bullet_bundle);
            }
        }
    }
}