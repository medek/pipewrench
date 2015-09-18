use nalgebra::{BaseFloat, FloatPnt, Pnt3, Pnt2, ApproxEq, Cast};
use num::Float;

pub enum CollisionShape<V,S> {
    Circle(Circle<S>),
    AABB2(AABB2<V>),
    AABB3(AABB3<V>)
}

pub struct Circle<S> {
    pub radius: S,
}

pub type Sphere<S> = Circle<S>;

pub struct AABB2<V> {
    corners: [V; 4]
}

pub struct AABB3<V> {
    corners: [V; 8]
}

pub trait Collision<C,V> {
    fn colliding(&self, pos: &V, other: &C, other_pos: &V) -> bool;
}

pub trait PointCollision<V> {
    fn point_inside(&self, pos: &V, point: &V) -> bool;
}

pub trait Intersection<V> {
    fn line_intersect(&self, pos: &V, start: &V, end: &V) -> bool;
}

//Circle implementations
impl<S> Collision<Circle<S>,Pnt3<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn colliding(&self, pos: &Pnt3<S>, other: &Circle<S>, other_pos: &Pnt3<S>) -> bool {
        pos.dist(other_pos) < self.radius + other.radius
    }
}

impl<S> Collision<Circle<S>, Pnt2<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn colliding(&self, pos: &Pnt2<S>, other: &Circle<S>, other_pos: &Pnt2<S>) -> bool {
        pos.dist(other_pos) < self.radius + other.radius
    }
}

impl<S> PointCollision<Pnt2<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn point_inside(&self, pos: &Pnt2<S>, point: &Pnt2<S>) -> bool {
        pos.dist(point) < self.radius
    }
}

impl<S> PointCollision<Pnt3<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn point_inside(&self, pos: &Pnt3<S>, point: &Pnt3<S>) -> bool {
        pos.dist(point) < self.radius
    }
}

impl<S> Intersection<Pnt2<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn line_intersect(&self, pos: &Pnt2<S>, start: &Pnt2<S>, end: &Pnt2<S>) -> bool {
        let dx = end.x - start.x;
        let dy = end.y - start.y;

        let a = dx.powi(2) + dy.powi(2);
        let b = <S as Cast<f64>>::from(2.0f64) * dx * (start.x - pos.x) + dy * (start.y - pos.y);
        let c = (start.x - pos.x).powi(2) + (start.y - pos.y).powi(2) - self.radius.powi(2);
        b.powi(2) - (<S as Cast<f64>>::from(4.0f64) * a * c) > S::zero()
    }
}

impl<S> Collision<CollisionShape<Pnt2<S>,S>, Pnt2<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn colliding(&self, pos: &Pnt2<S>, other: &CollisionShape<Pnt2<S>,S>, other_pos: &Pnt2<S>) -> bool {
        match other {
            &CollisionShape::Circle(ref c) => self.colliding(pos, c, other_pos),
            &CollisionShape::AABB2(ref aabb) => self.colliding(pos, aabb, other_pos),
            _ => false //TODO: unimplemented
        }
    }
}

impl<S> Collision<CollisionShape<Pnt3<S>,S>, Pnt3<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn colliding(&self, pos: &Pnt3<S>, other: &CollisionShape<Pnt3<S>,S>, other_pos: &Pnt3<S>) -> bool {
        match other {
            &CollisionShape::Circle(ref c) => self.colliding(pos, c, other_pos),
            _ => false //TODO: unimplemented
        }
    }
}

impl<S> Collision<AABB2<Pnt2<S>>, Pnt2<S>> for Circle<S> where S: BaseFloat + ApproxEq<S> {
    fn colliding(&self, pos: &Pnt2<S>, other: &AABB2<Pnt2<S>>, other_pos: &Pnt2<S>) -> bool {
        other.point_inside(other_pos, pos)
        || self.line_intersect(pos, &(*other_pos + other.corners[0].to_vec()), &(*other_pos + other.corners[1].to_vec()))
        || self.line_intersect(pos, &(*other_pos + other.corners[1].to_vec()), &(*other_pos + other.corners[2].to_vec()))
        || self.line_intersect(pos, &(*other_pos + other.corners[2].to_vec()), &(*other_pos + other.corners[3].to_vec()))
        || self.line_intersect(pos, &(*other_pos + other.corners[0].to_vec()), &(*other_pos + other.corners[3].to_vec()))
    }
}

// AABB2 implementations
impl<S> PointCollision<Pnt2<S>> for AABB2<Pnt2<S>> where S: BaseFloat + ApproxEq<S> {
    fn point_inside(&self, pos: &Pnt2<S>, point: &Pnt2<S>) -> bool {
        point.x > (self.corners[0].x + pos.x) && point.x < (self.corners[2].x + pos.x)
        && point.y < (self.corners[0].y + pos.y) && point.y > (self.corners[2].y + pos.y)
    }
}

impl<S> Collision<Circle<S>, Pnt2<S>> for AABB2<Pnt2<S>> where S: BaseFloat + ApproxEq<S> {
    fn colliding(&self, pos: &Pnt2<S>, other: &Circle<S>, other_pos: &Pnt2<S>) -> bool {
        other.colliding(other_pos, self, pos)
    }
}

impl<S> AABB2<Pnt2<S>> where S: BaseFloat + ApproxEq<S> {
    pub fn new(width: S, height: S) -> AABB2<Pnt2<S>> {
        AABB2 {
            corners: [
                //top left
                Pnt2::<S>::new(-width/Cast::from(2.0), height/<S as Cast<f64>>::from(2.0)),
                //top right
                Pnt2::<S>::new(width/<S as Cast<f64>>::from(2.0), height/<S as Cast<f64>>::from(2.0)),
                //bottom right
                Pnt2::<S>::new(width/<S as Cast<f64>>::from(2.0), -height/<S as Cast<f64>>::from(2.0)),
                //bottom left
                Pnt2::<S>::new(-width/<S as Cast<f64>>::from(2.0), -height/<S as Cast<f64>>::from(2.0)),
            ]
        }
    }
}

impl<S> CollisionShape<Pnt2<S>,S> where S: BaseFloat + ApproxEq<S> {
    pub fn point_inside(&self, pos: &Pnt2<S>, point: &Pnt2<S>) -> bool {
        match *self {
            CollisionShape::Circle(ref c) => c.point_inside(pos, point),
            CollisionShape::AABB2(ref c) => c.point_inside(pos, point),
            _ => false //TODO: unimplemented
        }
    }
}
