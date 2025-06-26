use bevy::prelude::*;

pub trait HitBoxTrait{
    fn is_in_hit_box(&self,lhs: Vec2, rhs: Vec2)->bool;
}

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct PointHitBox;
impl HitBoxTrait for PointHitBox{
    fn is_in_hit_box(&self,lhs: Vec2, rhs: Vec2)->bool {
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
    fn is_in_hit_box(&self,lhs: Vec2, rhs: Vec2)->bool {
        let temp = lhs + self.offset;
        temp.distance(rhs) <= self.radius
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

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct SquareHitBox;

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct RectangleHitBox;

#[derive(Component,Debug,Clone,Copy)]
pub enum HitBox{
    Point(PointHitBox),
    Circle(CircularHitBox),
    Square(SquareHitBox),
    Rectangle(RectangleHitBox)
}
impl Default for HitBox{
    fn default() -> Self {
        Self::Circle(CircularHitBox{offset:Vec2::ZERO, radius:1.0})
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