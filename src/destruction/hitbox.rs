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
pub struct ThreePointHitBox{
    a: Vec2,
    b: Vec2,
    c: Vec2
}
impl ThreePointHitBox{
    pub fn new(a: Vec2,b: Vec2,c: Vec2)->Self{
        Self { a, b, c}
    }
    fn is_inside_three_points(&self,point:&Vec2,a:&Vec2,b:&Vec2,c:&Vec2)->bool{
        let d1 = Self::sign_calculator(point, a, b);
        let d2 = Self::sign_calculator(point, b, c);
        let d3 = Self::sign_calculator(point, c, a);
        let negative = [d1,d2,d3].iter().any(|&v| v < 0.0);
        let positive = [d1,d2,d3].iter().any(|&v| v > 0.0);
        !(negative && positive)
    }
    fn sign_calculator(a:&Vec2, b:&Vec2, c:&Vec2)->f32{
        return (a.x - c.x) * (b.y - c.y) - (b.x - c.x)*(a.y-c.y)
    }
}
impl HitBoxTrait for ThreePointHitBox{
    fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
        let a = self.a + *lhs;
        let b = self.b + *lhs;
        let c = self.c + *lhs;
        self.is_inside_three_points(rhs, &a, &b, &c)
    }
}

#[derive(Component,Debug,Default,Clone,Copy)]
pub struct FourPointHitBox{
    top_left: Vec2,
    top_right: Vec2,
    bottom_left: Vec2,
    bottom_right: Vec2
}
#[allow(unused)]
impl FourPointHitBox{
    pub fn new(top_left: Vec2,top_right: Vec2,bottom_left: Vec2,bottom_right: Vec2)->Self{
        Self { top_left, top_right, bottom_left, bottom_right}
    }
    pub fn new_rectangle(top_left: Vec2, bottom_right: Vec2)->Self{
        let bottom_left = Vec2::new(top_left.x, bottom_right.y);
        let top_right = Vec2::new( bottom_right.x, top_left.y);
        Self::new(top_left, top_right, bottom_left, bottom_right)
    }
    pub fn new_square(top_left: Vec2)->Self{
        let bottom_right = -top_left;
        let top_right = Vec2::new( -top_left.x, top_left.y);
        let bottom_left = -top_right;
        Self::new(top_left, top_right, bottom_left, bottom_right)
    }
}
impl HitBoxTrait for FourPointHitBox{
    fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
        let (a,b,c, d) = (lhs+self.top_left, lhs+self.top_right, lhs+&self.bottom_left,  lhs+&self.bottom_right);
        let left_chiral = ThreePointHitBox::new(a, b, c).is_in_hit_box(lhs, rhs);
        let right_chiral = ThreePointHitBox::new(a, d, c).is_in_hit_box(lhs, rhs);
        left_chiral || right_chiral
    }
}
#[allow(unused)]
#[derive(Component,Debug,Clone,Copy)]
pub enum HitBox{
    Point(PointHitBox),
    Circle(CircularHitBox),
    Rectangle(FourPointHitBox),
    Triangle(ThreePointHitBox)
}
impl Default for HitBox{
    fn default() -> Self {
        Self::Circle(CircularHitBox{offset:Vec2::ZERO, radius:1.0})
    }
}
 impl HitBoxTrait for HitBox{
     fn is_in_hit_box(&self,lhs: &Vec2, rhs: &Vec2)->bool {
        match self{
            HitBox::Point(point_hit_box) => point_hit_box.is_in_hit_box(lhs,rhs),
            HitBox::Circle(circular_hit_box) => circular_hit_box.is_in_hit_box(lhs,rhs),
            HitBox::Rectangle(four_point_hit_box) => four_point_hit_box.is_in_hit_box(lhs,rhs),
            HitBox::Triangle(three_point_hit_box) => three_point_hit_box.is_in_hit_box(lhs,rhs),
        }
    }
}
