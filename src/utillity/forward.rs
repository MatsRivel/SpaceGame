use bevy::prelude::*;
pub trait ForwardUnit<T>{
    fn forward_unit_vector(&self)->T;
}

impl ForwardUnit<Vec3> for &Transform{
    fn forward_unit_vector(&self)->Vec3 {
        let rot = self.rotation;
        let (_,_,theta) = rot.to_euler(EulerRot::XYZ);
        let x = -f32::sin(theta);
        let y = f32::cos(theta);
        Vec2::new(x,y).extend(0.0)
    }
}