use std::rc::Rc;
use cgmath::BaseFloat as SpacialKey;
use super::{AABB2, Intersect, Intersection, Circle};
use cgmath::Point2 as Pnt2;

pub trait SpacialIndex {
    fn get_position<T: SpacialKey>(&self) -> Pnt2<T>;
}
#[derive(Debug)]
pub struct QuadTree<S, T> where T: SpacialIndex + Sized, S: SpacialKey {
    bucket: Vec<Rc<T>>,
    //NW, NE, SE, SW
    children: Option<[Box<QuadTree<S, T>>; 4]>,
    capacity: usize,
    volume: AABB2<S>
}

impl<'a, S, T> QuadTree<S, T> where T: SpacialIndex + Sized, S: SpacialKey {
    pub fn with_capacity(volume: AABB2<S>, capacity: usize) -> QuadTree<S, T> {
        QuadTree {
            bucket: Vec::with_capacity(capacity),
            children: None,
            capacity,
            volume
        }
    }

    fn subdivide(qt: &mut QuadTree<S, T>) {
        let min = qt.volume.tl;
        let max = qt.volume.br;

        let hw = (qt.volume.br.x - qt.volume.tl.x)/S::from(2.0).unwrap().abs();
        let hh = (qt.volume.br.y - qt.volume.tl.y)/S::from(2.0).unwrap().abs();

        qt.children = Some(
            [
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x, min.y), Pnt2::new(min.x + hw, min.y + hh)), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x + hw, min.y), Pnt2::new(max.x, min.y + hh)), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x + hw, min.y + hh), Pnt2::new(max.x, max.y)), qt.capacity)),
                Box::new(QuadTree::<S,T>::with_capacity(AABB2::<S>::new(Pnt2::new(min.x, min.y + hh), Pnt2::new(min.x + hw, max.y)), qt.capacity)),
            ]
        );
    }

    pub fn insert(&mut self, obj: &Rc<T>) -> bool {
        let pos = obj.get_position();

        if self.volume.intersection(&pos) == Intersection::Outside {
            return false
        }

        if self.bucket.len() == self.capacity {
            Self::subdivide(self);

            for _ in 0..self.bucket.len() {
                let val = self.bucket.pop().unwrap();
                self.insert(&val);
            }
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

    pub fn remove_key(&mut self, old: &Pnt2<S>) -> Option<Rc<T>> {
        if self.volume.intersection(&old) == Intersection::Outside {
            return None;
        }

        if self.children.is_none() {
            for i in 0..self.bucket.len() {
                if self.bucket[i].get_position() == *old {
                    return Some(self.bucket.remove(i))
                }
            }
        }

        for i in 0..4 {
            if let Some(ret) = self.children.as_mut().unwrap().get_mut(i).unwrap().remove_key(old) {
                    return Some(ret);
            }
        }
        None
    }

    pub fn update(&mut self, old: &Pnt2<S>, new: &Rc<T>) -> bool {
        if let Some(_) = self.remove_key(old) {
            return self.insert(new)
        }
        return false;
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

#[cfg(test)]
#[derive(Debug, PartialEq, Clone)]
struct TestEntity {
    pub pos: Pnt2<f32>,
}

#[cfg(test)]
impl TestEntity {
    pub fn new(p: Pnt2<f32>) -> TestEntity {
        TestEntity { pos: p }
    }
}

#[cfg(test)]
impl SpacialIndex for TestEntity {
    fn get_position<T: SpacialKey>(&self) -> Pnt2<T> {
        Pnt2::new(T::from(self.pos.x).unwrap(), T::from(self.pos.y).unwrap())
    }
}

#[test]
fn quad_tree() {
    let mut qt = QuadTree::<f32, TestEntity>::with_capacity(AABB2::new(Pnt2::new(-10.0, 10.0), Pnt2::new(10.0, -10.0)), 2);
    let mut entities = vec![Rc::new(TestEntity::new(Pnt2::new(-8.0, 8.0))), Rc::new(TestEntity::new(Pnt2::new(-9.0, 9.0))), Rc::new(TestEntity::new(Pnt2::new(2.0, 9.0)))];
    for i in 0..entities.len()-1 {
        qt.insert(&entities[i]);
    }

    assert_eq!(qt.bucket.len(), 2);

    qt.insert(&entities[2]);
    assert_eq!(qt.bucket.len(), 0);
    assert_eq!(qt.children.is_some(), true);
    assert_eq!(qt.children.as_ref().unwrap()[1].bucket.len(), 1);
    assert_eq!(qt.children.as_ref().unwrap()[1].bucket[0], entities[2]);
    assert_eq!(qt.children.as_ref().unwrap()[1].volume, AABB2::new(Pnt2::new(0.0, 10.0), Pnt2::new(10.0, 0.0)));

    let old_pos = entities[2].get_position();
    Rc::make_mut(&mut entities[2]).pos.x = -2.0;
    qt.update(&old_pos, &entities[2]);

    assert_eq!(qt.children.as_ref().unwrap()[0].children.as_ref().unwrap()[1].bucket.len(), 1);
    assert_eq!(qt.children.as_ref().unwrap()[0].children.as_ref().unwrap()[1].bucket[0], entities[2]);

    assert_eq!(qt.get_in_radius(&Circle::new(Pnt2::new(-13.0, -13.0), 4.0)), Some(entities.clone().into_iter().take(2).collect::<Vec<Rc<TestEntity>>>()));
    assert_eq!(qt.get_in_radius(&Circle::new(Pnt2::new(-13.0, -13.0), 12.0)), Some(entities));
}
