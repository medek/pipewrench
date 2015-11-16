use std::rc::Rc;
use super::super::Thingie as SpacialKey;
use super::{AABB2, Intersect, Intersection};
use nalgebra::Pnt2;

pub trait SpacialIndex {
    fn get_position<T: SpacialKey> (&self) -> Pnt2<T>;
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

        let (hw, hh) = (min[0]/S::from(2.0).unwrap(), max[1]/S::from(2.0).unwrap());
        qt.children = Some(
            [
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new([min[0], min[1]], [hw, hh]), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new([min[0] + hw, min[1]], [max[0], hh]), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new([min[0] + hw, min[1] + hh], [max[0], max[1]]), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new([min[0], min[1] + hh], [hw, max[1]]), qt.capacity)),
            ]
        );

        for _ in 0..qt.bucket.len() {
            let val = qt.bucket.pop().unwrap();
            qt.insert(&val);
        }
    }

    pub fn insert(&mut self, obj: &Rc<T>) -> bool {
        if self.bucket.len() == self.capacity {
            Self::subdivide(self);
        }

        let pos = obj.get_position();

        if self.volume.intersection(&pos) == Intersection::Outside {
            return false
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
}
