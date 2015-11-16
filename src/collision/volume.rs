use Thingie as Base;
use nalgebra::Pnt2;
use super::{Intersect, Intersection};

#[derive(Debug,Clone)]
pub struct AABB2<T: Base> {
    pub tl: [T;2],
    pub br: [T;2]
}

impl<T> AABB2<T> where T: Base {
    pub fn new(tl: [T;2], br: [T;2]) -> AABB2<T> {
        AABB2 {
            tl: tl,
            br: br
        }
    }
}

impl<S> Intersect<Pnt2<S>, S> for AABB2<S> where S: Base {
    fn intersection(&self, other: &Pnt2<S>) -> Intersection<S> {
        if other.x > self.tl[0] && other.x < self.br[0]
        && other.y < self.tl[1] && other.y > self.br[1] {
            return Intersection::Inside
        }
        Intersection::Outside
    }
}
