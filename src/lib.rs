#[macro_use]
#[cfg(feature = "window")]
extern crate glium;
#[cfg(feature = "window")]
extern crate glium_sdl2;
#[cfg(feature = "window")]
extern crate sdl2;
#[cfg(feature = "config")]
extern crate toml;
#[cfg(feature = "collision")]
extern crate cgmath;
#[macro_use]
extern crate thiserror;
#[macro_use]
mod macros;
#[cfg(feature = "window")]
mod window;
#[cfg(feature = "window")]
mod input;
mod storage;
mod cgmath_augment;
#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "collision")]
pub mod collision;

#[cfg(feature = "window")]
pub use window::*;
#[cfg(feature = "window")]
pub use input::*;
pub use storage::*;

