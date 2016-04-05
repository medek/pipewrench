use std::rc::Rc;
use super::super::Thingie as SpacialKey;
use super::{AABB2, Intersect, Intersection, Circle};
use nalgebra::Pnt2;

pub trait SpacialIndex {
    fn get_position<T: SpacialKey>(&self) -> Pnt2<T>;
}

pub struct QuadTree<S, T> where T: SpacialIndex + Sized, S: SpacialKey {
    bucket: Vec<Rc<T>>,
    //NW, NE, SE, SW
    children: Option<[Box<QuadTree<S, T>>; 4]>,
    capacity: usize,
    volume: AABB2<S>
}

impl<S, T> QuadTree<S, T> where T: SpacialIndex + Sized, S: SpacialKey {
    pub fn with_capacity(volume: AABB2<S>, capacity: usize) -> QuadTree<S, T> {
        QuadTree {
            bucket: Vec::with_capacity(capacity),
            children: None,
            capacity: capacity,
            volume: volume
        }
    }

    fn subdivide(qt: &mut QuadTree<S, T>) {
        let min = qt.volume.tl;
        let max = qt.volume.br;

        let (hw, hh) = (min.x/S::from(2.0).unwrap(), max.y/S::from(2.0).unwrap());
        qt.children = Some(
            [
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x, min.y), Pnt2::new(hw, hh)), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x + hw, min.y), Pnt2::new(max.x, hh)), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x + hw, min.y + hh), Pnt2::new(max.x, max.y)), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x, min.y + hh), Pnt2::new(hw, max.y)), qt.capacity)),
            ]
        );

        for _ in 0..qt.bucket.len() {
            let val = qt.bucket.pop().unwrap();
            qt.insert(&val);
        }
    }

    pub fn insert(&mut self, obj: &Rc<T>) -> bool {
        let pos = obj.get_position();

        if self.volume.intersection(&pos) == Intersection::Outside {
            return false
        }

        if self.bucket.len() == self.capacity {
            Self::subdivide(self);
        }

        match self.children {
            Some(ref mut quad) => for node in quad.iter_mut() {
                if node.insert(obj) {
                    return true
                }
            },
            None => {
                self.bucket.push(obj.clone());
                return true
            }
        }

        false
    }

    pub fn get_in_radius(&self, at: &Circle<S>) -> Option<Vec<Rc<T>>> {
        match at.intersection(&self.volume) {
            Intersection::Outside => return None,
            _ => {}
        }
        let mut ret:Vec<Rc<T>> = Vec::new();
        if let Some(ref c) = self.children {
            for quad in c.into_iter() {
                if let Some(items) = quad.get_in_radius(at) {
                    ret.extend_from_slice(&items[..]);
                }
            }
        }
        else {
            for x in &self.bucket {
                ret.push(x.clone())
            }
        }

        return Some(ret);
    }
}
