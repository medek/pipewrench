use glium_sdl2::{DisplayBuild, SDL2Facade};
use glium::Frame;
use glium::texture::RawImage2d;
use sdl2::VideoSubsystem;
use image::RgbaImage;
use result::*;
use std::path::Path;
use std::fs::File;
use std::rc::Rc;

extern crate image;

pub struct Window {
    display: Rc<SDL2Facade>,
}

impl Window {
    pub fn new(vid: &VideoSubsystem, title: &str, width: u32, height: u32) -> PWResult<Window> {
        let display = try!(vid.window(title, width, height).build_glium());
        Ok(Window {
            display: Rc::new(display)
        })
    }

    pub fn display(&self) -> Rc<SDL2Facade> {
        self.display.clone()
    }

    pub fn draw(&self) -> Frame {
        self.display.draw()
    }

    pub fn screenshoti<'a>(&self, file: &Path) -> PWResult<()> {
        let image:RawImage2d<'a, u8> = self.display.read_front_buffer();
        if let Some(image) = RgbaImage::from_raw(image.width, image.height, image.data.as_ref().to_vec()) {
            try!(image.save(file));
            return Ok(());
        }
        return Err(PWError::GenericError("RgbaImage couldn't read raw pixels...".to_string()))
    }
}
