use Thingie as Base;
use nalgebra::Pnt2;
use super::{Intersect, Intersection};

#[derive(Debug,Clone)]
pub struct AABB2<T: Base> {
    pub tl: Pnt2<T>,
    pub br: Pnt2<T>
}

impl<T> AABB2<T> where T: Base {
    pub fn new(tl: Pnt2<T>, br: Pnt2<T>) -> AABB2<T> {
        AABB2 {
            tl: tl,
            br: br
        }
    }
}

impl<S> Intersect<Pnt2<S>, S> for AABB2<S> where S: Base {
    fn intersection(&self, other: &Pnt2<S>) -> Intersection<S> {
        if self.contains(other) {
            return Intersection::Inside
        }
        else if self.intersects(other) {
            return Intersection::Intersects(other.clone(), None)
        }

        Intersection::Outside
    }

    fn intersects(&self, other: &Pnt2<S>) -> bool {
        other.x == self.tl.x || other.y == self.tl.y ||
        other.x == self.br.x || other.y == self.br.y
    }

    fn contains(&self, other: &Pnt2<S>) -> bool {
        other.x > self.tl.x && other.x < self.br.x
        && other.y < self.tl.y && other.y > self.br.y
    }
}
