use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;
use std::path::Path;

pub trait SizedError: std::error::Error + Sized {}

#[derive(Debug,Error)]
pub enum StorageError {
    #[error("Storage already has key named \"{0}\"")]
    StorageOccupied(String),
    #[error("Error: {source}")]
    Error {
        #[from]
        source: Box<dyn std::error::Error>
    }
}

pub type LoaderFunction<T>= dyn Fn(&Path) -> Result<T, Box<dyn std::error::Error>>;

pub struct Storage<T> {
    data: HashMap<String, Rc<T>>,
    loader: Box<LoaderFunction<T>>,
}

impl<T> Storage<T> where T: Sized {
    pub fn new(loader: Box<LoaderFunction<T>>) -> Storage<T> {
        Storage {
            data: HashMap::new(),
            loader
        }
    }

    ///! consume data and return a reference counted version of it
    pub fn add(&mut self, path: &String, data: T) -> Result<Rc<T>, StorageError> {
        match self.data.entry(path.clone()) {
            Entry::Occupied(_) => {
                Err(StorageError::StorageOccupied(path.clone()))
            },
            Entry::Vacant(slot) => {
                let new = Rc::new(data);
                slot.insert(new.clone());
                Ok(new)
            }
        }
    }

    pub fn load(&mut self, path: &String) -> Result<Rc<T>, StorageError> {
        match self.data.entry(path.clone()) {
            Entry::Occupied(slot) => {
                Ok(slot.get().clone())
            }
            Entry::Vacant(slot) => {
                let data = (self.loader)(&Path::new(path))?;
                let ret = Rc::new(data);
                slot.insert(ret.clone());
                Ok(ret)
            }
        }
    }

    pub fn has(&self, path: &String) -> bool {
        self.data.contains_key(path)
    }

    pub fn get(&self, path: &String) -> Option<Rc<T>> {
        if self.has(path) {
            Some(self.data.get(path).unwrap().clone())
        }
        else {
            None
        }
    }

    pub fn drop(&mut self, path: &String) -> Option<Rc<T>> {
        self.data.remove(path)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}
