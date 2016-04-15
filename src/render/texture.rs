extern crate image;
use std::path::Path;
use std::fs::File;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use image::{GenericImage, ImageFormat};
use glium::texture::{RawImage2d, Texture2d};
use super::prelude::*;
use result::*;

pub struct BasicTextureStorage {
    data: HashMap<String, Rc<Texture2d>>,
    facade: Rc<SDL2Facade>,
}

impl BasicTextureStorage {
    pub fn new(facade: &Rc<SDL2Facade>) -> BasicTextureStorage {
        BasicTextureStorage {
            data: HashMap::new(),
            facade: facade.clone()
        }
    }

    ///! consume data and return a reference counted version of it
    pub fn add(&mut self, path: &String, data: Texture2d) -> PWResult<Rc<Texture2d>> {
        match self.data.entry(path.clone()) {
            Entry::Occupied(_) => {
                Err(PWError::StorageOccupied(path.clone()))
            },
            Entry::Vacant(slot) => {
                let new = Rc::new(data);
                slot.insert(new.clone());
                Ok(new)
            }
        }
    }

    fn load_file(facade: &SDL2Facade, path: &Path) -> PWResult<Texture2d> {
        let mut ext = ImageFormat::PNG;
        if let Some(fext) = path.extension() {
            ext = match fext.to_str().unwrap() {
                "png" => ImageFormat::PNG,
                "tga" => ImageFormat::TGA,
                "jpeg" | "jpg" => ImageFormat::JPEG,
                "PNG" => ImageFormat::PNG,
                "TGA" => ImageFormat::TGA,
                "JPEG" | "JPG"  => ImageFormat::JPEG,
                _ => ext
            }
        }
        let f = try!(File::open(path));
        let image = try!(image::load(f, ext)).to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
        let tex = try!(Texture2d::new(facade, image));
        Ok(tex)
    }

    pub fn load(&mut self, path: &String) -> PWResult<Rc<Texture2d>> {
        match self.data.entry(path.clone()) {
            Entry::Occupied(slot) => {
                Ok(slot.get().clone())
            }
            Entry::Vacant(slot) => {
                let facade = self.facade.clone();
                let data = try!(Self::load_file(facade.as_ref(), &Path::new(path)));
                let ret = Rc::new(data);
                slot.insert(ret.clone());
                Ok(ret)
            }
        }
    }

    pub fn has(&self, path: &String) -> bool {
        self.data.contains_key(path)
    }

    pub fn get(&self, path: &String) -> Option<Rc<Texture2d>> {
        if self.has(path) {
            Some(self.data.get(path).unwrap().clone())
        }
        else {
            None
        }
    }

    pub fn drop(&mut self, path: &String) -> Option<Rc<Texture2d>> {
        self.data.remove(path)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}
