use bevy::prelude::*;

use crate::entities::firearm::bullet::{Bullet, BulletFactory, MakeBullet};
use crate::entities::object::Object;
use crate::movement::velocity::linear_velocity::Velocity;
use crate::utillity::timing::SelfDestructTimer;

pub trait BulletMakerRequirements: MakeBullet +Default +Clone +Send +Sync +'static +From::<Option<Handle<Image>>>{}

#[derive(Component, Default)]
pub struct HasGunTag;

#[derive(Component, Clone)]
#[require(HasGunTag, Object)]
pub struct Gun<T: BulletMakerRequirements>{
    pub attack_speed: f32,
    pub magazine_size: u32,
    pub reload_speed: u32,
    pub bullet_factory: T,

}
impl <T: BulletMakerRequirements>Gun<T>{
    pub fn new(attack_speed: f32, magazine_size: u32, reload_speed: u32, bullet_image: Option<Handle<Image>>) -> Self{
        Self { attack_speed, magazine_size, reload_speed, bullet_factory: T::from(bullet_image)}
    }
}
impl <T: BulletMakerRequirements>MakeBullet for Gun<T>{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_position: &Vec2, adjusted_rotation: &Quat) -> (Bullet, Velocity, bevy::prelude::Transform, bevy::prelude::Sprite, SelfDestructTimer) {
        self.bullet_factory.make_bullet_bundle(possible_velocity, parent_position, adjusted_rotation)
    }
}

impl <T: BulletMakerRequirements>Default for Gun<T>{
    fn default() -> Self {
        Self { 
            attack_speed: 1.0, 
            magazine_size: 5,
            reload_speed: 5,
            bullet_factory: T::default(),
        }
    }
}


pub fn fire_bullet<PlayerIdentification:Component, BulletCreator:BulletMakerRequirements>(
    mut commands: Commands, 
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    gun_query: Query<(&GlobalTransform,&Gun::<BulletCreator>)>,
    wielder_query: Query<(&Transform, &Children, Option<&Velocity>, Option<&PlayerIdentification>), With<HasGunTag>>){
    if !keyboard_input.just_pressed(KeyCode::Space) { return;}
    for (transform, children, possible_velocity, possible_player_tag) in wielder_query{
        if possible_player_tag.is_some() && !keyboard_input.just_pressed(KeyCode::Space){
            // Players only fire when space is pressed.
            continue; 
        }else if possible_player_tag.is_none(){
            panic!("\n --- Not Implemented! ---\nWe haven't considered other entities shooting yet...\n")
        }
        let wielder_rotation = transform.rotation;
        for child in children.iter() {
            if let Ok((gun_transform, gun)) = gun_query.get(child) {
                let gun_global_pos = (gun_transform.translation()).truncate();
                let gun_rotation = gun_transform.rotation();
                let final_rotation = gun_rotation.rotate_towards(transform.rotation, gun_rotation.angle_between(wielder_rotation));
                let bullet_bundle = gun.make_bullet_bundle(possible_velocity, &gun_global_pos, &final_rotation);
                commands.spawn(bullet_bundle);
            }
        }
    }
}