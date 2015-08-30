#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate toml;

#[macro_use]
mod macros;
mod result;
mod window;
mod input;
pub mod config;

pub use result::*;
pub use window::*;
pub use input::*;
