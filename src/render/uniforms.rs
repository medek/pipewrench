///! Usable version of glium::uniforms::UniformsStorage

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use glium::uniforms::{UniformValue, Uniforms};
use result::{PWResult, PWError};

#[derive(Clone)]
pub struct UniformsStorage<'n> {
    uniforms: HashMap<String, UniformValue<'n>>,
}

impl<'n> UniformsStorage<'n> {
    pub fn new(s: String, val: UniformValue<'n>) -> UniformsStorage<'n> {
        let mut ret = UniformsStorage {
            uniforms: HashMap::new(),
        };
        ret.add(s, val).unwrap();
        ret
    }

    pub fn add(&mut self, s: String, val: UniformValue<'n>) -> PWResult<()> {
        match self.uniforms.entry(s.clone()) {
            Entry::Occupied(_) => {
                Err(PWError::StorageOccupied(s))
            },
            Entry::Vacant(slot) => {
                slot.insert(val);
                Ok(())
            }
        }
    }

    pub fn overwrite(&mut self, s: String, val: UniformValue<'n>) {
        match self.uniforms.entry(s) {
            Entry::Occupied(slot) => {
                let v = slot.into_mut();
                *v = val;
            },
            Entry::Vacant(slot) => {
                slot.insert(val);
            }
        }
    }

    pub fn clear(&mut self) {
        self.uniforms.clear();
    }
}


impl<'n> Uniforms for UniformsStorage<'n> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut func: F) {
        for x in self.uniforms.iter() {
            func(x.0.as_ref(), x.1.clone());
        }
    }
}
