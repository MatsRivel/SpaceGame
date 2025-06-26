use bevy::prelude::*;

use crate::destruction::hitbox::HitBox;
pub mod hitbox{
    use bevy::prelude::*;
    pub trait HitBoxTrait{
        fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool;
    }
    
    #[derive(Component,Debug,Default,Clone,Copy)]
    pub struct PointHitBox;
    impl HitBoxTrait for PointHitBox{
        fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
            lhs == rhs
        }
    }
    
    #[derive(Component,Debug,Default,Clone,Copy)]
    pub struct CircularHitBox{
        pub offset: Vec2,
        pub radius: f32
    }
    impl CircularHitBox{
        pub fn new(offset:Vec2,radius:f32)->Self{
            Self{offset,radius}
        }
    }
    impl HitBoxTrait for CircularHitBox{
        fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
            let temp = lhs + self.offset;
            temp.distance(*rhs) <= self.radius
        }
    }
    #[derive(Component,Debug,Default,Clone,Copy)]
    pub struct FourPointHitBox{
        top_left: Vec2,
        top_right: Vec2,
        bottom_left: Vec2,
        bottom_right: Vec2
    }
    impl FourPointHitBox{
        pub fn new(top_left: Vec2,top_right: Vec2,bottom_left: Vec2,bottom_right: Vec2)->Self{
            Self { top_left, top_right, bottom_left, bottom_right}
        }
    }
    impl HitBoxTrait for FourPointHitBox{
        fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
            let (a,b,c, d) = (lhs+self.top_left, lhs+self.top_right, lhs+&self.bottom_left,  lhs+&self.bottom_right);
            is_inside_three_points(&rhs, &a, &b, &c) || is_inside_three_points(&rhs, &a, &b, &d)
        }
    }
    
    #[derive(Component,Debug,Default,Clone,Copy)]
    pub struct RectangleHitBox{
        top_left: Vec2,
        bottom_right: Vec2
    }
    impl RectangleHitBox{
        pub fn new(top_left: Vec2, bottom_right: Vec2)->Self{
            Self { top_left, bottom_right}
        }
    }
    impl HitBoxTrait for RectangleHitBox{
        fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
            let top_right = Vec2::new(self.bottom_right.x,self.top_left.y);
            let bottom_left = Vec2::new(self.top_left.x, self.bottom_right.y);
            let (a,b,c, d) = (lhs+self.top_left, lhs+top_right, lhs+bottom_left,  lhs+&self.bottom_right);
            is_inside_three_points(&rhs, &a, &b, &c) || is_inside_three_points(&rhs, &a, &b, &d)
        }
    }
    #[derive(Component,Debug,Clone,Copy)]
    pub enum HitBox{
        Point(PointHitBox),
        Circle(CircularHitBox),
        Rectangle(RectangleHitBox),
    }
    impl Default for HitBox{
        fn default() -> Self {
            Self::Circle(CircularHitBox{offset:Vec2::ZERO, radius:1.0})
        }
    }

    fn sign_calculator(a:&Vec2, b:&Vec2, c:&Vec2)->f32{
        return (a.x - c.x) * (b.y - c.y) - (b.x - c.x)*(a.y-c.y)
    }
    fn is_inside_three_points(point: &Vec2, a:&Vec2, b:&Vec2, c:&Vec2)->bool{
        let d1 = sign_calculator(point, a, b);
        let d2 = sign_calculator(point, b, c);
        let d3 = sign_calculator(point, c, a);
        let negative = [d1,d2,d3].iter().any(|&v| v < 0.0);
        let positive = [d1,d2,d3].iter().any(|&v| v > 0.0);
        !(negative && positive)
    }
}
#[derive(Component,Debug,Default,Clone,Copy)]
#[require(HitBox)]
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