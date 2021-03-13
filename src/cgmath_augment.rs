use cgmath::{Point2, Vector2};

pub trait Cross<T, S> {
    fn cross(&self, other: &T) -> S;
}

pub trait Dot<T, S> {
    fn dot(&self, other: &T) -> S;
}

//stupid type rules won't let me add std::ops::Add for f32/f64
pub trait AddScalar<T, S> {
    fn add_scalar(self, rhs: S) -> T;
}

impl<S> Cross<Point2<S>, S> for Point2<S> where S: cgmath::BaseFloat {
    fn cross(&self, other: &Point2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<S> Cross<Vector2<S>, S> for Vector2<S> where S: cgmath::BaseFloat {
    fn cross(&self, other: &Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<S> Dot<Point2<S>, S> for Point2<S> where S: cgmath::BaseFloat {
    fn dot(&self, other: &Point2<S>) -> S {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<S> Dot<Vector2<S>, S> for Vector2<S> where S: cgmath::BaseFloat {
    fn dot(&self, other: &Vector2<S>) -> S {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<S> AddScalar<Point2<S>, S> for Point2<S>  where S: cgmath::BaseFloat {
    fn add_scalar(self, rhs: S) -> Point2<S> {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}
