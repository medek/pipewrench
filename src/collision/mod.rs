mod volume;
mod quadtree;

use nalgebra::Pnt2;
use super::Thingie;
pub use self::volume::*;
pub use self::quadtree::*;

#[derive(Debug,Clone,PartialEq)]
pub enum Intersection<S> where S: Thingie {
    Outside,
    Inside,
    Intersects(Pnt2<S>, Option<Pnt2<S>>)
}

pub trait Intersect<T, S> where S: Thingie {
    fn intersection(&self, other: &T) -> Intersection<S>;
}
