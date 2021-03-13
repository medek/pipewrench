use glium_sdl2::{DisplayBuild, SDL2Facade};
use glium::Frame;
use sdl2::VideoSubsystem;
use std::rc::Rc;

extern crate image;
#[derive(Debug, Error)]
pub enum WindowError {
    #[error("SDL Error: {}", source)]
    SDLError {
        #[from]
        source: glium_sdl2::GliumSdl2Error
    },
    #[error("IOError: {}", source)]
    IOError {
        #[from]
        source: std::io::Error
    },
    #[error("ImageError: {}", source)]
    ImageError {
        #[from]
        source: image::ImageError,
    },
    #[error("Failure to create screenshot from raw buffer")]
    ScreenshotFailure
}
pub struct Window {
    display: Rc<SDL2Facade>,
}

impl Window {
    pub fn new(vid: &VideoSubsystem, title: &str, width: u32, height: u32) -> Result<Window, WindowError> {
        let display = vid.window(title, width, height).build_glium()?;
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
}
