use Thingie as Base;
use nalgebra::{Pnt2, Vec2, Cross, Dot};
use super::{Intersect, Intersection};

pub struct Circle<S: Base> {
    pub pos: Pnt2<S>,
    pub radius: S
}

///Lines go from a to b
pub struct Line<S: Base> {
    pub a: Pnt2<S>,
    pub b: Pnt2<S>
}

impl<S> Line<S> where S: Base {
    pub fn new(a: Pnt2<S>, b: Pnt2<S>) -> Line<S> {
        Line {
            a: a,
            b: b
        }
    }
}

impl<S> Intersect<Line<S>, S> for Line<S> where S: Base {
    fn intersection(&self, other: &Line<S>) -> Intersection<S> {
        let r = Vec2::new(self.b.x - self.a.x, self.b.y - self.a.y);
        let s = Vec2::new(other.b.x - other.a.x, other.b.y - other.a.y);

        //cross returns a Vec1... wtf?!
        let unum = (other.a - self.a).cross(&r).x;
        let denom = r.cross(&s).x;

        if unum == S::zero() && denom == S::zero() {
            let t0 = (other.a - self.a).dot(&r) / r.dot(&r);
            let t1 = (other.a + s - self.a).dot(&r) / r.dot(&r);
            //do they overlap
            // s and r go in opposite directions so it's t1 -> t0

            if s.dot(&r) < S::zero() && t1 <= S::one() && S::zero() <= t0 {
                //TODO: fix
                return Intersection::Overlap(other.a.clone(), self.b.clone())
            } else if t0 <= S::one() && S::zero() <= t1 {
                //TODO: fix
                return Intersection::Overlap(other.a.clone(), self.b.clone())
            }

            return Intersection::Outside
        }

        if denom == S::zero() {
            //lines are parallel
            return Intersection::Parallel
        }

        let u = unum / denom;
        let t = (other.a - self.a).cross(&s).x / denom;

        if t >= S::zero() && t <= S::one() && u >= S::zero() && u <= S::one() {
            return Intersection::Intersects(self.a + (r * t), None)
        }

        return Intersection::Outside
    }

    fn intersects(&self, other: &Line<S>) -> bool {
        match self.intersection(other) {
            Intersection::Overlap(_, _) => true,
            Intersection::Intersects(_, _) => true,
            _ => false
        }
    }

    fn contains(&self, other: &Line<S>) -> bool {
        false
    }
}

#[test]
fn line_intersects() {
    // no intersect
    assert_eq!(Line::new(Pnt2::new(0.0,0.0), Pnt2::new(2.0, 8.0))
               .intersects(&Line::new(Pnt2::new(8.0, 0.0), Pnt2::new(0.0, 20.0))), false);
    // intersect
    assert_eq!(Line::new(Pnt2::new(0.0,10.0), Pnt2::new(2.0, 0.0))
               .intersects(&Line::new(Pnt2::new(10.0, 0.0), Pnt2::new(0.0, 5.0))), true);
    // parallel, vertical
    assert_eq!(Line::new(Pnt2::new(0.0,0.0), Pnt2::new(0.0, 10.0))
               .intersects(&Line::new(Pnt2::new(2.0, 0.0), Pnt2::new(2.0, 10.0))), false);
    // parallel, diagonal
    assert_eq!(Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0))
               .intersects(&Line::new(Pnt2::new(2.0, 0.0), Pnt2::new(7.0, 5.0))), false);
    //collinear, overlap
    assert_eq!(Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0))
               .intersects(&Line::new(Pnt2::new(2.0, 2.0), Pnt2::new(7.0, 7.0))), true);
    //collinear, no overlap
    assert_eq!(Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0))
               .intersects(&Line::new(Pnt2::new(7.0, 7.0), Pnt2::new(10.0, 10.0))), false);
}

#[test]
fn line_intersection() {
    let mut l1 = Line::new(Pnt2::new(-5.0, 5.0), Pnt2::new(-1.0, 5.0));
    let mut l2 = Line::new(Pnt2::new(0.0, -5.0), Pnt2::new(0.0, 5.0));

    // no intersection
    assert_eq!(l1.intersection(&l2), Intersection::Outside);

    l1.b.x = 5.0;
    // intersection
    assert_eq!(l1.intersection(&l2), Intersection::Intersects(Pnt2 {x: 0.0, y: 5.0}, None));

    l1 = Line::new(Pnt2::new(0.0,0.0), Pnt2::new(0.0, 10.0));
    l2 = Line::new(Pnt2::new(2.0, 0.0), Pnt2::new(2.0, 10.0));
    // parallel, vertical
    assert_eq!(l1.intersection(&l2), Intersection::Parallel);

    l1 = Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0));
    l2 = Line::new(Pnt2::new(2.0, 0.0), Pnt2::new(7.0, 5.0));

    // parallel, diagonal
    assert_eq!(l1.intersection(&l2), Intersection::Parallel);


    l1 = Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0));
    l2 = Line::new(Pnt2::new(2.0, 2.0), Pnt2::new(7.0, 7.0));

    //collinear, overlap
    assert_eq!(l1.intersection(&l2), Intersection::Overlap(l2.a.clone(), l1.b.clone()));
    l1 = Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0));
    l2 = Line::new(Pnt2::new(7.0, 7.0), Pnt2::new(10.0, 10.0));

    //collinear, no overlap
    assert_eq!(l1.intersection(&l2), Intersection::Outside);
}
