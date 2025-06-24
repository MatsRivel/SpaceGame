use bevy::prelude::*;

#[derive(Clone)]
pub struct BulletFactory{
    pub damage: f32,
    pub radius: f32,
    pub mass: f32,
    pub lifetime: f32,
    pub speed: f32,
    pub bullet_image: Option<Handle<Image>>
}
impl From<Option<Handle<Image>>> for BulletFactory{
    fn from(value: Option<Handle<Image>>) -> Self {
        Self{bullet_image:value, ..Default::default()}
    }
}
impl BulletMakerRequirements for BulletFactory{}

impl MakeBullet for BulletFactory{
    fn make_bullet_bundle(&self, possible_velocity: Option<&Velocity>, parent_position: &Vec2, adjusted_rotation: &Quat)->(Bullet, Velocity, bevy::prelude::Transform, bevy::prelude::Sprite, SelfDestructTimer){
        let bullet = Bullet::new(self.damage, self.radius, self.mass);
        let parent_velocity = **possible_velocity.unwrap_or(&Velocity::new(Vec2::ONE));
        let rotation_adjusted_movement = adjusted_rotation.mul_vec3(Vec3::Y).truncate()*self.speed;
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
            Transform::from_translation(parent_position.extend(0.0)).with_rotation(*adjusted_rotation),
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