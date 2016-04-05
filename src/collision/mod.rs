mod quadtree;
mod primitive;

use nalgebra::{Pnt2};
use super::Thingie;
pub use self::quadtree::*;
pub use self::primitive::*;

pub trait Intersect<T, S> where S: Thingie {
    fn intersection(&self, other: &T) -> Intersection<S>;
    fn intersects(&self, other: &T) -> bool;
    fn contains(&self, other: &T) -> bool;
}

#[derive(Debug,Clone,PartialEq)]
pub enum Intersection<S> where S: Thingie {
    Outside,
    Inside,
    InverseContain,
    Parallel,
    Overlap(Pnt2<S>, Pnt2<S>),
    Intersects(Pnt2<S>, Option<Pnt2<S>>),
    IntersectsN(Vec<Pnt2<S>>)
}

impl<S> Intersection<S> where S: Thingie {
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

