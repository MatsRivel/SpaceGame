use bevy::prelude::*;

use crate::movement::gravity::gravity_2d::GravityAffected;
use crate::entities::object::Object;
use crate::movement::linear_movement_2d::LinearSpeedModifier;
use crate::movement::velocity::linear_velocity::Velocity;
use crate::utillity::timing::SelfDestructTimer;

#[derive(Component)]
#[require(BulletFactory)]
pub struct Gun{
    pub attack_speed: f32,
    pub magazine_size: u32,
    pub reload_speed: u32,
}
impl Gun{
    pub fn new(attack_speed: f32, magazine_size: u32, reload_speed: u32) -> Self{
        Self { attack_speed, magazine_size, reload_speed }
    }
}

impl Default for Gun{
    fn default() -> Self {
        Self { 
            attack_speed: 1.0, 
            magazine_size: 5,
            reload_speed: 5,
        }
    }
}
#[derive(Component)]
pub struct BulletFactory{
    pub damage: f32,
    pub radius: f32,
    pub mass: f32,
    pub lifetime: f32,
    pub speed: f32
}
impl BulletFactory{
    pub fn make_bullet(&self)->Bullet{
        Bullet::new(self.damage, self.radius, self.mass)
    }
}
impl Default for BulletFactory{
    fn default() -> Self {
        Self {
            damage: 1.0, 
            radius: 1.0, 
            mass: 1.0,
            lifetime: 6.0,
            speed: 100.0
        }
    }
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
pub fn fire_bullet(mut commands: Commands, asset_server: Res<AssetServer>, keyboard_input: Res<ButtonInput<KeyCode>>, query: Query<(&Transform, &BulletFactory, Option<&Velocity>), With<Gun>>){
    if !keyboard_input.just_pressed(KeyCode::Space) { return;}
    for (transform, bullet_factory, optional_velocity) in query{
        let bullet = bullet_factory.make_bullet();
        let origin = transform.translation;
        let forward = origin + transform.rotation * Vec3::Y;
        let position = origin + forward.normalize() * 1.0;
        let asset_path = r"sprites\FX\bullet\bullet1.png";
        let image = asset_server.load(asset_path);
        let bullet_speed = forward.truncate()*bullet_factory.speed;
        let bullet_velocity = match optional_velocity{
           Some(&parent_velocity) => Velocity::new(bullet_speed + *parent_velocity),
           None => Velocity::new(bullet_speed)
        };
        commands.spawn((
            bullet,
            Transform::from_translation(position).with_rotation(transform.rotation * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)), // Rotate the image 90 degrees CC.
            bullet_velocity,
            LinearSpeedModifier::new(bullet_factory.speed),
            Sprite::from_image(image),
            SelfDestructTimer::new(bullet_factory.lifetime)
        ));
    }
}