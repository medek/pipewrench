use Thingie as Base;
use nalgebra::{Pnt2, Vec2, Cross, Dot};
use super::{Intersect, Intersection};

#[derive(Debug,Clone)]
pub struct Circle<S: Base> {
    pub pos: Pnt2<S>,
    pub radius: S
}

///Lines go from a to b
#[derive(Debug,Clone)]
pub struct Line<S: Base> {
    pub a: Pnt2<S>,
    pub b: Pnt2<S>
}

#[derive(Debug,Clone)]
pub struct AABB2<T: Base> {
    pub tl: Pnt2<T>,
    pub br: Pnt2<T>
}

impl<S> Line<S> where S: Base {
    pub fn new(a: Pnt2<S>, b: Pnt2<S>) -> Line<S> {
        Line {
            a: a,
            b: b
        }
    }
}

impl<S> Circle<S> where S: Base {
    pub fn new(pos: Pnt2<S>, radius: S) -> Circle<S> {
        Circle {
            pos: pos,
            radius: radius
        }
    }
}

impl<S> AABB2<S> where S: Base {
    pub fn new(tl: Pnt2<S>, br: Pnt2<S>) -> AABB2<S> {
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

impl<S> Intersect<Line<S>, S> for Line<S> where S: Base {
    fn intersection(&self, other: &Line<S>) -> Intersection<S> {
        let r = Vec2::new(self.b.x - self.a.x, self.b.y - self.a.y);
        let s = Vec2::new(other.b.x - other.a.x, other.b.y - other.a.y);

        //cross returns a Vec1... wtf?!
        let unum = (other.a - self.a).cross(&r).x;
        let denom = r.cross(&s).x;

        if unum == S::zero() && denom == S::zero() {
            let t0 = (other.a - self.a).dot(&r) / r.dot(&r);
            let t1 = t0 + s.dot(&r) / r.dot(&r);
            //do they overlap
            // s and r go in opposite directions so s dot r < 0 it's t1 -> t0
            if s.dot(&r) < S::zero() && t1 <= S::one() && S::zero() <= t0 {
                return Intersection::Overlap(self.a + (r * t1), self.a + (r * t0))
            } else if t0 <= S::one() && S::zero() <= t1 {
                return Intersection::Overlap(self.a + (r * t0), self.a + (r * t1))
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
        //this function should probably just return false
        match self.intersection(other) {
            Intersection::Overlap(a, b) => {
                a.x >= self.a.x && a.y >= self.a.y && b.x <= self.b.x && b.y <= self.b.y
            },
            _ => false
        }
    }
}

impl<S> Intersect<Circle<S>, S> for Line<S> where S: Base {
    fn intersection(&self, other: &Circle<S>) -> Intersection<S> {
        other.intersection(self)
    }

    fn intersects(&self, other: &Circle<S>) -> bool {
        other.intersects(self)
    }

    fn contains(&self, other: &Circle<S>) -> bool {
        !other.contains(self)
    }
}

impl<S> Intersect<Line<S>, S> for Circle<S> where S: Base {
    fn intersection(&self, other: &Line<S>) -> Intersection<S> {
        let two = S::from(2.0).unwrap();
        let d = other.b - other.a;

        if d.x == S::zero() && d.y == S::zero() { //line has zero length
            //I could be an asshole and let it pretend it's a tangent and return NaNs,
            //I guess I'm just that nice
            return self.intersection(&other.a)
        }

        let f = other.a - self.pos;
        let a = d.dot(&d);
        let b = S::from(2.0).unwrap()*f.dot(&d);
        let c = f.dot(&f) - self.radius.powf(two);

        let mut dis = b.powf(two) - (S::from(4.0).unwrap() * a * c);
        if dis < S::zero() {
            return Intersection::Outside
        }
        else if dis == S::zero() {
            dis = dis.sqrt();

            let t0 = (-b - dis)/(two * a);
            return Intersection::Intersects(other.a + (d * t0), None)
        }
        else {
            dis = dis.sqrt();

            let t0 = (-b - dis)/(two * a);
            let t1 = (-b + dis)/(two * a);

            if t0 < S::zero() && t1 <= S::one() { //A inside circle
                return Intersection::Intersects(other.a + (d * t1), None)
            }
            else if t1 > S::one() && t0 >= S::zero() { //B inside circle
                return Intersection::Intersects(other.a + (d * t0), None)
            }
            else if t1 > S::one() && t0 < S::zero() { //all inside
                return Intersection::Inside
            }
            else if t0 > S::zero() && t0 < S::one() && t1 > S::zero() && t1 < S::one() { //two intersections
                return Intersection::Intersects(other.a + (d * t0), Some(other.a + (d * t1)))
            }
        }
        Intersection::Outside
    }

    fn intersects(&self, other: &Line<S>) -> bool {
        match self.intersection(other) {
            Intersection::Intersects(_, _) => true,
            _ => false
        }
    }

    fn contains(&self, other: &Line<S>) -> bool {
        match self.intersection(other) {
            Intersection::Inside => true,
            _ => false
        }
    }
}

impl<S> Intersect<Pnt2<S>, S> for Circle<S> where S: Base {
    fn intersection(&self, other: &Pnt2<S>) -> Intersection<S> {
        let d = *other - self.pos;

        if d.x.abs() < self.radius && d.y.abs() < self.radius {
            return Intersection::Inside
        }
        else if d.x.abs() > self.radius && d.y.abs() > self.radius {
            return Intersection::Outside
        }
        else {
            return Intersection::Intersects(other.clone(), None)
        }
    }

    fn intersects(&self, other: &Pnt2<S>) -> bool {
        let d = *other - self.pos;
        //should probably add an epsilon-y type check here but eh
        d.x.abs() == self.radius && d.y.abs() == self.radius
    }

    fn contains(&self, other: &Pnt2<S>) -> bool {
        match self.intersection(other) {
            Intersection::Inside => true,
            _ => false
        }
    }
}

impl<S> Intersect<AABB2<S>, S> for Circle<S> where S: Base {
    fn intersection(&self, other: &AABB2<S>) -> Intersection<S> {
        let mut l:Line<S> = Line::new(other.tl, other.br);
        let mut p:Vec<Pnt2<S>> = Vec::new();
        let mut outside = 0;

        match self.intersection(&l) {
            Intersection::Inside => return Intersection::Inside,
            _ => {}
        }

        l = Line::new(other.tl, Pnt2::new(other.br.x, other.tl.y)); //top

        match self.intersection(&l) {
            Intersection::Intersects(a, b) => {
                p.push(a);
                if b.is_some() {
                    p.push(b.unwrap());
                }
            },
            Intersection::Outside => outside += 1,
            _ => {}
        }

        l = Line::new(Pnt2::new(other.br.x, other.tl.y), other.br); //right

        match self.intersection(&l) {
            Intersection::Intersects(a, b) => {
                p.push(a);
                if b.is_some() {
                    p.push(b.unwrap());
                }
            },
            Intersection::Outside => outside += 1,
            _ => {}
        }

        l = Line::new(other.br, Pnt2::new(other.tl.x, other.br.y)); //bottom

        match self.intersection(&l) {
            Intersection::Intersects(a, b) => {
                p.push(a);
                if b.is_some() {
                    p.push(b.unwrap());
                }
            },
            Intersection::Outside => outside += 1,
            _ => {}
        }

        l = Line::new(Pnt2::new(other.tl.x, other.br.y), other.tl); //left

        match self.intersection(&l) {
            Intersection::Intersects(a, b) => {
                p.push(a);
                if b.is_some() {
                    p.push(b.unwrap());
                }
            },
            Intersection::Outside => outside += 1,
            _ => {}
        }

        if outside == 4 {
            if other.intersection(&self.pos) == Intersection::Inside {
                return Intersection::InverseContain;
            }
            return Intersection::Outside;
        }

        Intersection::IntersectsN(p)
    }

    fn intersects(&self, other: &AABB2<S>) -> bool {
        match self.intersection(other) {
            Intersection::IntersectsN(_) => true,
            _ => false
        }
    }

    fn contains(&self, other: &AABB2<S>) -> bool {
        match self.intersection(other) {
            Intersection::Inside => true,
            _ => false
        }
    }
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
    assert_eq!(l1.intersection(&l2), Intersection::Overlap(l2.a.clone(), l2.b.clone()));
    l1 = Line::new(Pnt2::new(0.0,0.0), Pnt2::new(5.0, 5.0));
    l2 = Line::new(Pnt2::new(7.0, 7.0), Pnt2::new(10.0, 10.0));

    //collinear, no overlap
    assert_eq!(l1.intersection(&l2), Intersection::Outside);

    l1.b.x = 15.0;
    l1.b.y = 15.0;
    //it is right but there's stupid floating point mess
    assert_eq!(l1.intersection(&l2), Intersection::Overlap(Pnt2 {x: 7.0, y: 7.0 }, Pnt2 {x: 10.000000000000002, y: 10.000000000000002}));
    assert_eq!(l1.contains(&l2), true)
}

#[test]
fn circle_line_intersection() {
    let circle = Circle::new(Pnt2::new(3.0,1.0), 6.0);
    let mut line = Line::new(Pnt2::new(-3.0, -3.0), Pnt2::new(-3.0, 3.0));

    //tangent
    assert_eq!(circle.intersection(&line), Intersection::Intersects(Pnt2 {x: -3.0, y: 1.0}, None));

    line.a = Pnt2::new(-6.0, 1.0);
    line.b = Pnt2::new(12.0, 1.0);

    //intersection A -> B
    assert_eq!(circle.intersection(&line), Intersection::Intersects(Pnt2 {x: -3.0, y: 1.0}, Some(Pnt2 {x: 9.0, y: 1.0})));

    let temp = line.a;
    line.a = line.b;
    line.b = temp;

    //intersecion B -> A
    assert_eq!(circle.intersection(&line), Intersection::Intersects(Pnt2 {x: 9.0, y: 1.0}, Some(Pnt2 {x: -3.0, y: 1.0})));

    line.b = line.a;
    line.a = circle.pos.clone();
    //intersection A inside circle
    assert_eq!(circle.intersection(&line), Intersection::Intersects(Pnt2 {x: 9.0, y: 1.0}, None));

    line.a = Pnt2::new(-6.0, 1.0);
    line.b = circle.pos;

    //intersection B inside circle
    assert_eq!(circle.intersection(&line), Intersection::Intersects(Pnt2 {x: -3.0, y: 1.0}, None));


    line.a = circle.pos + (circle.radius/2.0);
    line.b = circle.pos;

    //line completely inside
    assert_eq!(circle.intersection(&line), Intersection::Inside);

    line.a = Pnt2::new(-20.0, 20.0);
    line.b = Pnt2::new(20.0, 20.0);

    //line completely outside
    assert_eq!(circle.intersection(&line), Intersection::Outside);

    line.b = line.a.clone();
    //zero length line outside
    assert_eq!(circle.intersection(&line), Intersection::Outside);

    line.a = circle.pos.clone();
    line.b = line.a.clone();

    //zero length line inside
    assert_eq!(circle.intersection(&line), Intersection::Inside);
}

#[test]
fn circle_aabb2_intersection() {
    let mut circle = Circle::new(Pnt2::new(0.0, 0.0), 5.0);
    let mut aabb = AABB2::new(Pnt2::new(-5.0, 5.0), Pnt2::new(5.0, -5.0));

    // four faces
    assert_eq!(circle.intersection(&aabb),
               Intersection::IntersectsN(vec![Pnt2::new(0.0, 5.0),
                                              Pnt2::new(5.0, 0.0),
                                              Pnt2::new(0.0, -5.0),
                                              Pnt2::new(-5.0, 0.0)
                                             ]));

    // four corners
    circle.radius = 7.0710678118654755; //thanks pythagoras

    assert_eq!(circle.intersection(&aabb),
               Intersection::IntersectsN(vec![Pnt2::new(5.0, 5.0),
                                              Pnt2::new(5.0, -5.0),
                                              Pnt2::new(-5.0, -5.0),
                                              Pnt2::new(-5.0, 5.0)
                                             ]));
    //inside
    circle.radius = 10.0;
    assert_eq!(circle.intersection(&aabb), Intersection::Inside);

    //circle inside aabb
    circle.radius = 3.0;

    assert_eq!(circle.intersection(&aabb), Intersection::InverseContain);
}

