#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate toml;
extern crate num;
extern crate cgmath;
#[macro_use]
extern crate thiserror;
#[macro_use]
mod macros;
mod window;
mod input;
mod storage;
mod cgmath_augment;
pub mod config;
#[cfg(feature = "collision")]
pub mod collision;

pub use window::*;
pub use input::*;
pub use storage::*;

