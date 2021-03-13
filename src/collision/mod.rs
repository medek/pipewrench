mod quadtree;
mod primitive;
use cgmath::Point2 as Pnt2;
use cgmath::BaseFloat;
pub use self::quadtree::*;
pub use self::primitive::*;

pub trait Intersect<T, S> where S: BaseFloat {
    fn intersection(&self, other: &T) -> Intersection<S>;
    fn intersects(&self, other: &T) -> bool;
    fn contains(&self, other: &T) -> bool;
}

#[derive(Debug,Clone,PartialEq)]
pub enum Intersection<S> where S: BaseFloat {
    Outside,
    Inside,
    InverseContain,
    Parallel,
    Overlap(Pnt2<S>, Pnt2<S>),
    Intersects(Pnt2<S>, Option<Pnt2<S>>),
    IntersectsN(Vec<Pnt2<S>>)
}

impl<S> Intersection<S> where S: BaseFloat {
    pub fn inside(&self) -> bool {
        match *self {
            Intersection::Inside => true,
            _ => false
        }
    }

    pub fn outside(&self) -> bool {
        match *self {
            Intersection::Outside => true,
            _ => false
        }
    }

    pub fn intersects(&self) -> bool {
        match *self {
            Intersection::Intersects(_, _) => true,
            _ => false
        }
    }
}

