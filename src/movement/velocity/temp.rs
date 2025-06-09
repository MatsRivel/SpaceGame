pub mod linear_movement_2d{
    use bevy::prelude::*;
    use std::ops::Mul;
    #[derive(Component,Default, Clone, Copy)]
    pub struct LinearSpeedModifier(f32);
    impl LinearSpeedModifier{
        pub fn new(speed: f32)->Self{
            Self(speed)
        }
    }
    impl std::ops::Deref for LinearSpeedModifier {
        type Target = f32;
    
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl Mul<LinearSpeedModifier> for Vec2{
        type Output = Vec2;
    
        fn mul(self, rhs: LinearSpeedModifier) -> Self::Output {
            self * *rhs
        }
    }
}

pub mod rotation_2d{
    use bevy::prelude::*;
    use std::ops::Mul;

    #[derive(Component,Default)]
    pub struct RotationalSpeedModifier(f32);
    
    impl RotationalSpeedModifier{
        pub fn new(speed: f32)->Self{
            Self(speed)
        }
    }
    
    impl std::ops::Deref for RotationalSpeedModifier {
        type Target = f32;
    
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl Mul<f32> for &RotationalSpeedModifier{
        type Output = f32;
    
        fn mul(self, rhs: f32) -> Self::Output {
            self.0 * rhs
        }
    }

}

pub mod velocity_2d{
    pub fn conservation_of_linear_momentum(
        time: Res<Time>,
        mut query: Query<(&mut Transform, &Velocity, &LinearSpeedModifier)>,
    ){
        for (mut transform, &velocity, &speed_mod) in query.iter_mut() {
            // let rotation_adjusted_movement = transform.rotation.mul_vec3(velocity.0.extend(0.0));
            let rotation_adjusted_movement: Vec3 = velocity.into();
            transform.translation += rotation_adjusted_movement * speed_mod * time.delta_secs();
        }
    }
    pub fn conservation_of_rotational_momentum(
        time: Res<Time>,
        mut query: Query<(&mut Transform,  &RotationalSpeedModifier)>,
    ){
        for (mut transform, &speed_mod) in query.iter_mut() {
            // let rotation_adjusted_movement = transform.rotation.mul_vec3(velocity.0.extend(0.0));
            let rotation_adjusted_movement: Vec3 = velocity.into();
            transform.rotation += rotation_adjusted_movement * speed_mod * time.delta_secs();
        }
    }

}