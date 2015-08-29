use glium_sdl2::{DisplayBuild, SDL2Facade};
use glium::Frame;
use sdl2::{Sdl, VideoSubsystem};
use result::*;
use std::error::Error;
use std::path::Path;
use std::fs::File;

extern crate image;

pub struct Window {
    video: VideoSubsystem,
    display: SDL2Facade,
}

impl Window {
    pub fn new(vid: VideoSubsystem, title: &str, width: u32, height: u32) -> PWResult<Window> {
        let display = try!(vid.window(title, width, height).build_glium());
        Ok(Window {
            video: vid,
            display: display
        })
    }

    pub fn draw(&self) -> Frame {
        self.display.draw()
    }

    pub fn screenshot(&self, file: &Path) -> PWResult<()> {
        let image:image::DynamicImage = self.display.read_front_buffer();
        let mut output = try!(File::create(file));

        try!(image.save(&mut output, image::ImageFormat::PNG));
        Ok(())
    }
}
