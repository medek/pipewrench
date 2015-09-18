#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate toml;
extern crate nalgebra;
extern crate num;
#[macro_use]
mod macros;
mod result;
mod window;
mod input;

pub mod config;
pub mod collision;

pub use result::*;
pub use window::*;
pub use input::*;
